use serde::{Deserialize, Serialize};

use super::data::Tuple;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Page {
    pub tuples: Vec<Tuple>,
}

pub fn new_page(tuple: Tuple) -> Page {
    return Page {
        tuples: Vec::from([tuple]),
    };
}

pub fn new_pageid(_table_name: String) -> i64 {
    return 0;
}
