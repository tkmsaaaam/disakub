use std::collections::BTreeMap;

use crate::meta::{
    btree::{new_b_tree, BTree, Node},
    lru::{new_lru, Lru},
};

use super::{
    data::Tuple,
    disk::{new_disk_manager, DiskManager},
    page::{new_page, new_pageid, Page},
};

#[derive(Clone, Debug)]
pub struct Storage {
    pub buffer: BufferPool,
    pub disk: DiskManager,
    pub prefix: String,
}

#[derive(Clone, Debug)]
pub struct BufferPool {
    pub lru: Lru,
    pub btree: BTreeMap<String, BTree>,
}

pub fn new_storage(home: String) -> Storage {
    return Storage {
        buffer: new_buffer_pool(),
        disk: new_disk_manager(),
        prefix: home,
    };
}

impl Storage {
    pub fn create_index(mut self, index_name: String) -> Storage {
        let btree = new_b_tree();
        self.buffer.btree.insert(index_name, btree.clone());
        return self;
    }

    pub fn insert_tuple(mut self, tablename: String, tuple: Tuple) -> Storage {
        self = self.insert_page(tablename, tuple);
        return self;
    }

    pub fn insert_index(self, index_name: String, item: Tuple) {
        let btree = self.read_index(index_name.clone());
        btree.insert(item);
    }

    pub fn read_index(self, index_name: String) -> BTree {
        match self.buffer.read_index(index_name) {
            Some(b) => return b,
            None => {
                return BTree {
                    top: Node { items: Vec::new() },
                    length: 0,
                }
            }
        };
        // TODO: fixme
    }

    pub fn insert_page(mut self, tablename: String, tuple: Tuple) -> Storage {
        let page = new_page(tuple);
        let page_id = new_pageid(tablename.clone());
        let res = self
            .buffer
            .put_page(tablename.clone(), page_id, page.clone());
        self.buffer = res.0;
        self.clone()
            .disk
            .persist(self.clone().prefix, tablename, page_id, res.2);
        return self;
    }

    pub fn read_tuple(self, tablename: String, i: i64) -> Tuple {
        let page_id = self.buffer.clone().to_page_id(i);
        let page = self.read_page(tablename, page_id);
        // let j = i % 32;
        return page.tuples.first().unwrap().clone();
    }

    fn read_page(self, tablename: String, page_id: i64) -> Page {
        let _page = self.buffer.read_page(tablename.clone(), page_id);

        let page_d = self.disk.fetch_page(self.prefix, tablename, page_id);

        // todo: fixme
        return page_d;
    }
}

pub fn new_buffer_pool() -> BufferPool {
    let mut btree = BTreeMap::new();
    btree.insert(
        String::new(),
        BTree {
            top: Node { items: Vec::new() },
            length: 0,
        },
    );
    return BufferPool {
        lru: new_lru(1000),
        btree: btree,
    };
}
