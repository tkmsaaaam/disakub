use crate::meta::btree::BTree;

use super::{page::Page, storage::BufferPool};

struct BufferTag {}

#[derive(Clone, Debug)]
pub struct PageDescriptor {
    dirty: bool,
    page: Page,
}

impl BufferPool {
    pub fn put_page(
        mut self,
        table_name: String,
        page_id: i64,
        page: Page,
    ) -> (BufferPool, bool, Page) {
        let buffer_tag = new_buffer_tag(table_name.clone(), page_id);
        let page_descriptor = PageDescriptor { dirty: false, page };

        let hash = buffer_tag.hash();
        let res = self.lru.insert(hash, page_descriptor);
        self.lru = res.0;
        return (self, res.1.value.dirty, res.1.value.page);
    }

    pub fn read_index(self, index_name: String) -> Option<BTree> {
        return self.btree.get(&index_name).cloned();
    }

    pub fn to_page_id(self, _to_id: i64) -> i64 {
        return 0;
    }

    pub fn read_page(self, table_name: String, to_id: i64) -> Page {
        let page_id = self.clone().to_page_id(to_id);
        let buffer_tag = new_buffer_tag(table_name, page_id);

        let hash = buffer_tag.hash();
        let page = self.lru.get(hash);
        return page.value.page;
    }
}

impl BufferTag {
    fn hash(self) -> u8 {
        return 0;
    }
}

fn new_buffer_tag(_table_name: String, _page_id: i64) -> BufferTag {
    return BufferTag {};
}
