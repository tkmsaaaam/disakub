use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

use super::page::Page;

#[derive(Clone, Debug)]
pub struct DiskManager {}

impl DiskManager {
    pub fn persist(self, dir_name: String, _table_name: String, page_id: i64, page: Page) {
        let file_name = page_id.to_string();
        let f = File::create(dir_name + "/" + &file_name + ".json").unwrap();
        let writer = BufWriter::new(f);
        let _ = serde_json::to_writer_pretty(writer, &page);
    }

    pub fn fetch_page(self, dir_name: String, _table_name: String, page_id: i64) -> Page {
        let file_name = page_id.to_string();
        let f = File::open(dir_name + "/" + &file_name + ".json").unwrap();
        let reader = BufReader::new(f);
        let page: Page = serde_json::from_reader(reader).unwrap();
        return page;
    }
}

pub fn new_disk_manager() -> DiskManager {
    return DiskManager {};
}
