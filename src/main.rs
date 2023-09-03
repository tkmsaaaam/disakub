mod db;
mod meta;
mod query;
mod storage;

use std::{thread, time};

pub use crate::db::disakub::new_disakub;
use crate::db::http::new_api_server;
pub use crate::storage::catalog::*;

fn main() {
    let disakub = db::disakub::new_disakub();
    disakub.clone().init();

    new_api_server(disakub).host();

    client();
    thread::sleep(time::Duration::from_secs(600));
}

fn client() {
    println!("DB is started");
    let mut input = String::new();
    let _i = std::io::stdin().read_line(&mut input).ok();
    let _query = input.trim().to_string();
    let url = String::from("http://127.0.0.1:7878/execute?q=") + &input;
    let _res = async {
        let _r = reqwest::get(url);
    };
}
