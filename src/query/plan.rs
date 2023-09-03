use crate::storage::{data::Tuple, storage::Storage};

use super::analyze::{Query, SelectQuery};

pub struct Planner {
    query: Query,
}

pub struct Plan {
    pub scanners: Scanner,
}

pub struct Scanner {
    pub scan: Scan,
}

pub struct Scan {
    pub table_name: String,
    pub index_name: String,
}

pub fn new_planner(query: Query) -> Planner {
    return Planner { query: query };
}

impl Planner {
    pub fn plan_main(self) -> Plan {
        match self.query.clone().select_query {
            Some(select_query) => self.plan_select(select_query.clone()),
            None => {
                return Plan {
                    scanners: Scanner {
                        scan: Scan {
                            table_name: String::new(),
                            index_name: String::new(),
                        },
                    },
                }
            }
        }
    }

    fn plan_select(self, select_query: SelectQuery) -> Plan {
        let mut pk = String::new();
        for column in select_query.from.get(0).unwrap().clone().columns {
            if column.primary == true {
                pk = column.name;
                break;
            }
        }
        return Plan {
            scanners: Scanner {
                scan: Scan {
                    table_name: select_query.from.get(0).unwrap().clone().name,
                    index_name: select_query.from.get(0).unwrap().clone().name + "_" + &pk,
                },
            },
        };
    }
}

impl Scan {
    pub fn scan(self, storage: Storage) -> Vec<Tuple> {
        let tuple = storage.read_tuple(self.table_name, 0);
        return Vec::from([tuple]);
    }
}
