use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Schema {
    pub table_name: String,
    pub column_names: Vec<String>,
    pub column_types: Vec<String>,
    pub primary_key: String,
}
pub struct ResultSet {
    pub message: String,
}
#[derive(Clone)]
pub struct Column {
    pub name: String,
    pub column_type: String,
    pub primary: bool,
}
#[derive(Clone)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
}

pub fn new_schema(
    table_name: String,
    column_names: Vec<String>,
    column_types: Vec<String>,
    primary_key: String,
) -> Schema {
    return Schema {
        table_name: table_name,
        column_names: column_names,
        column_types: column_types,
        primary_key: primary_key,
    };
}

pub fn new_with_message(message: String) -> ResultSet {
    return ResultSet { message: message };
}
