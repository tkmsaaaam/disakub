use std::collections::HashMap;

use crate::storage::bufpool::PageDescriptor;

#[derive(Clone, Debug)]
pub struct Lru {
    pub cap: i64,
    pub evict_list: Vec<Entry>,
    pub items: HashMap<u8, Entry>,
}

#[derive(Clone, Debug)]
pub struct Entry {
    pub value: PageDescriptor,
}

impl Lru {
    pub fn insert(mut self, key: u8, value: PageDescriptor) -> (Lru, Entry) {
        let element = Entry { value };
        self.evict_list.push(element.clone());
        self.items.insert(key, element);
        return (self.clone(), self.evict_list.first().unwrap().clone());
    }

    pub fn get(self, key: u8) -> Entry {
        return self.items.get(&key).unwrap().clone();
    }
}

pub fn new_lru(cap: i64) -> Lru {
    return Lru {
        cap,
        evict_list: Vec::new(),
        items: HashMap::new(),
    };
}
