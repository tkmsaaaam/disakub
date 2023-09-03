use crate::{
    meta::meta::{new_with_message, ResultSet},
    storage::{storage::Storage, tuple::new_tuple},
    Catalog,
};

use super::{
    analyze::{CreateTableQuery, InsertQuery, Query, SelectQuery},
    plan::Plan,
};

#[derive(Clone, Debug)]
pub struct Executor {
    storage: Storage,
    pub catalog: Catalog,
}

pub fn new_executor(storage: Storage, catalog: Catalog) -> Executor {
    return Executor {
        storage: storage,
        catalog: catalog,
    };
}

impl Executor {
    pub fn execute_main(self, query: Query, plan: Plan) -> (Catalog, Storage, ResultSet) {
        if query.create_table_query.is_some() {
            match query.create_table_query {
                Some(q) => {
                    let result = self.create_table(q);
                    return (result.0.catalog, result.0.storage, result.1);
                }
                None => {
                    return (
                        self.catalog,
                        self.storage,
                        ResultSet {
                            message: String::new(),
                        },
                    )
                }
            }
        } else if query.select_query.is_some() {
            match query.select_query {
                Some(q) => {
                    return {
                        let res = self.select_table(q, plan);
                        (res.0.catalog, res.0.storage, res.1)
                    }
                }
                None => {
                    return (
                        self.catalog,
                        self.storage,
                        ResultSet {
                            message: String::new(),
                        },
                    )
                }
            }
        } else if query.insert_query.is_some() {
            match query.insert_query {
                Some(q) => {
                    return {
                        let res = self.insert_table(q);
                        (res.0.catalog, res.0.storage, res.1)
                    }
                }
                None => {
                    return (
                        self.catalog,
                        self.storage,
                        ResultSet {
                            message: String::new(),
                        },
                    )
                }
            }
        } else {
            return (
                self.catalog,
                self.storage,
                ResultSet {
                    message: String::new(),
                },
            );
        }
    }

    fn create_table(mut self, query: CreateTableQuery) -> (Executor, ResultSet) {
        self.catalog = self.catalog.add(query.schema.clone());
        let index_name = query.schema.table_name.clone() + "_" + &query.schema.primary_key.clone();
        self.storage = self.storage.create_index(index_name);
        let message = query.schema.table_name.clone() + " was created as Table";
        return (self, new_with_message(message));
    }

    fn select_table(self, query: SelectQuery, plan: Plan) -> (Executor, ResultSet) {
        let tupls = plan.scanners.scan.scan(self.storage.clone());
        let mut values = Vec::new();
        for tuple in tupls {
            for column in query.columns.clone() {
                values.push(column.name);
                values.push(tuple.data[0].string.clone());
            }
        }
        println!("{:#?}", values);
        return (self, new_with_message(String::new()));
    }

    fn insert_table(mut self, query: InsertQuery) -> (Executor, ResultSet) {
        let tuple = new_tuple(0, query.values);
        self.storage = self
            .storage
            .clone()
            .insert_tuple(query.table.name, tuple.clone());
        self.storage.clone().insert_index(query.index, tuple);
        return (self, new_with_message(String::from("inserted")));
    }
}
