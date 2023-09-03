use super::data::{Tuple, TupleData};

pub fn new_tuple(min_tx_id: i64, values: Vec<String>) -> Tuple {
    let mut tuple_datas = Vec::new();
    for value in values {
        let data = TupleData {
            tuple_type: 0,
            number: 0,
            string: value,
        };
        tuple_datas.push(data);
    }
    return Tuple {
        min_tx_id,
        max_tx_id: min_tx_id,
        data: tuple_datas,
    };
}
