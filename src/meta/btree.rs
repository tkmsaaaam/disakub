use crate::storage::data::Tuple;

#[derive(Clone, Debug)]
pub struct BTree {
    pub(crate) top: Node,
    pub(crate) length: i64,
}
#[derive(Clone, Debug)]
pub struct Node {
    pub(crate) items: Vec<Item>,
}

#[derive(Clone, Debug)]
pub struct Item {}

impl BTree {
    pub fn insert(mut self, _item: Tuple) -> BTree {
        self.length += 1;
        return self;
    }

    pub fn get(self, key: Item) -> Item {
        return self.top.get(key);
    }
}

impl Node {
    fn get(self, _key: Item) -> Item {
        return self.items[0].clone();
    }
}

pub fn new_b_tree() -> BTree {
    let node = Node { items: Vec::new() };
    return BTree {
        top: node,
        length: 0,
    };
}
