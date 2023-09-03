use crate::{
    meta::meta::{new_schema, Column, Schema, Table},
    Catalog,
};

use super::parse::Statement;

pub struct Analyzer {
    catalog: Catalog,
}

#[derive(Clone)]
pub struct Query {
    pub create_table_query: Option<CreateTableQuery>,
    pub select_query: Option<SelectQuery>,
    pub insert_query: Option<InsertQuery>,
}

#[derive(Clone)]
pub struct CreateTableQuery {
    pub schema: Schema,
}

#[derive(Clone)]
pub struct SelectQuery {
    pub columns: Vec<Column>,
    pub from: Vec<Table>,
}

#[derive(Clone)]
pub struct InsertQuery {
    pub table: Table,
    pub values: Vec<String>,
    pub index: String,
}

pub fn new_analyzer(catalog: Catalog) -> Analyzer {
    return Analyzer { catalog: catalog };
}

impl Analyzer {
    pub fn analyze_main(self, statement: Statement) -> Query {
        if statement.create_statement.column_names.len() != 0 {
            return self.analyze_create_table(statement);
        } else if statement.select_statement.column_names.len() != 0 {
            return self.analyze_select(statement);
        } else {
            return self.analyze_insert(statement);
        }
    }

    fn analyze_select(self, statement: Statement) -> Query {
        let column = Column {
            name: statement.select_statement.column_names[0].clone(),
            column_type: String::from("String"),
            primary: false,
        };
        let columns = Vec::from([column]);

        let table = Table {
            name: statement.select_statement.from[0].clone(),
            columns: columns.clone(),
        };
        let tables = Vec::from([table]);
        return Query {
            create_table_query: None,
            select_query: Some(SelectQuery {
                columns,
                from: tables,
            }),
            insert_query: None,
        };
    }

    fn analyze_create_table(self, statement: Statement) -> Query {
        let schema = new_schema(
            statement.create_statement.table_name,
            statement.create_statement.column_names,
            statement.create_statement.column_types,
            statement.create_statement.primary_key,
        );

        let create_table_query = CreateTableQuery { schema: schema };
        return Query {
            create_table_query: Some(create_table_query),
            select_query: None,
            insert_query: None,
        };
    }

    fn analyze_insert(self, statement: Statement) -> Query {
        let schema = self
            .catalog
            .fetch_schema(statement.insert_statement.table.clone());
        let table = Table {
            name: statement.insert_statement.table.clone(),
            columns: Vec::new(),
        };
        let insert_query = InsertQuery {
            table: table,
            values: statement.insert_statement.values,
            index: statement.insert_statement.table + "_" + &schema.primary_key,
        };
        return Query {
            create_table_query: None,
            select_query: None,
            insert_query: Some(insert_query),
        };
    }
}
