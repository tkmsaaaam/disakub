use std::fs::File;
use std::io::{BufReader, BufWriter};

use crate::meta::meta::Schema;

#[derive(Debug, Clone)]
pub struct Catalog {
    pub schemas: Vec<Schema>,
}

impl Catalog {
    pub fn add(mut self, schema: Schema) -> Catalog {
        self.schemas.push(schema);
        return self;
    }
    pub fn fetch_schema(self, table_name: String) -> Schema {
        for ele in self.schemas {
            if ele.table_name == table_name {
                return ele;
            }
        }
        return Schema {
            table_name,
            column_names: Vec::new(),
            column_types: Vec::new(),
            primary_key: String::new(),
        };
    }
}

pub fn load_catalog(catalog_path: String) -> Result<Catalog, Box<dyn std::error::Error>> {
    let f = File::open(catalog_path + "/catalog.json").unwrap();

    let reader = BufReader::new(f);
    let schemas: Vec<Schema> = serde_json::from_reader(reader).unwrap();
    Ok(Catalog { schemas })
}

pub fn save_catalog(path: String, catalog: Catalog) {
    let f = File::create(path + "/catalog.json").unwrap();
    let writer = BufWriter::new(f);
    let _ = serde_json::to_writer_pretty(writer, &catalog.schemas);
}
