use super::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
}

pub struct Statement {
    pub create_statement: CreateStatement,
    pub select_statement: SelectStatement,
    pub insert_statement: InsertStatement,
}

pub struct CreateStatement {
    pub table_name: String,
    pub column_names: Vec<String>,
    pub column_types: Vec<String>,
    pub primary_key: String,
}

pub struct SelectStatement {
    pub column_names: Vec<String>,
    pub from: Vec<String>,
    pub wheres: Vec<String>,
}

#[derive(Clone)]
pub struct InsertStatement {
    pub table: String,
    pub values: Vec<String>,
}

pub fn new_parser(tokens: Vec<Token>) -> Parser {
    return Parser { tokens };
}

impl Parser {
    pub fn parse(self) -> Statement {
        if self.tokens[0].kind.eq(&0) {
            return self.create_table_statement();
        } else if self.tokens[0].kind.eq(&1) {
            return self.select_statement();
        }
        return self.insert_statement();
    }

    fn create_table_statement(self) -> Statement {
        let table = 0;
        self.expect(table);
        let column = Vec::from([String::from("column_z")]);
        let column_type = Vec::from([String::from("String")]);
        let create_statement = CreateStatement {
            table_name: String::from("table_z"),
            column_names: column,
            column_types: column_type,
            primary_key: String::from("column_z"),
        };
        let insert_statement = InsertStatement {
            table: String::new(),
            values: Vec::new(),
        };
        return Statement {
            create_statement: create_statement,
            select_statement: SelectStatement {
                column_names: Vec::new(),
                from: Vec::new(),
                wheres: Vec::new(),
            },
            insert_statement: insert_statement,
        };
    }

    fn select_statement(self) -> Statement {
        let column_names = Vec::from([String::from("column_z")]);
        let from = Vec::from([String::from("table_z")]);
        let select_statement = SelectStatement {
            column_names,
            from,
            wheres: Vec::new(),
        };
        let insert_statement = InsertStatement {
            table: String::new(),
            values: Vec::new(),
        };
        return Statement {
            create_statement: CreateStatement {
                table_name: String::new(),
                column_names: Vec::new(),
                column_types: Vec::new(),
                primary_key: String::new(),
            },
            select_statement: select_statement,
            insert_statement: insert_statement,
        };
    }

    fn insert_statement(self) -> Statement {
        let table_name = String::from("table_z");
        let values = Vec::from([String::from("value_z")]);
        return Statement {
            create_statement: CreateStatement {
                table_name: String::new(),
                column_names: Vec::new(),
                column_types: Vec::new(),
                primary_key: String::new(),
            },
            select_statement: SelectStatement {
                column_names: Vec::new(),
                from: Vec::new(),
                wheres: Vec::new(),
            },
            insert_statement: InsertStatement {
                table: table_name,
                values: values,
            },
        };
    }

    fn expect(self, _kind: i64) -> Token {
        // if parser.tokens[parser.position].kind == kind {
        //   token = parser.tokens[parser.position];
        //   parser.position =+ 1;
        //   return token;
        //   }
        return self.tokens[0].clone();
    }
}
