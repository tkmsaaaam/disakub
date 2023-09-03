use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TupleData {
    pub tuple_type: i64,
    pub number: i64,
    pub string: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tuple {
    pub min_tx_id: i64,
    pub max_tx_id: i64,
    pub data: Vec<TupleData>,
}
