use std::{
    io::Read,
    net::{TcpListener, TcpStream},
    thread,
};

use crate::{storage::storage::Storage, Catalog};

use super::disakub::Disakub;

#[derive(Clone, Debug)]
pub struct ApiServer {
    pub db: Disakub,
}

impl ApiServer {
    pub fn host(self) {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        let cloned_self = self.clone();

        thread::spawn(move || {
            let mut abc = cloned_self.clone();
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                abc = thread::spawn({
                    let mut api_server = abc.clone();
                    move || {
                        let res = api_server.clone().handle_connection(stream);
                        api_server.db.catalog = res.0;
                        api_server.db.storage = res.1;
                        return api_server;
                    }
                })
                .join()
                .unwrap();
            }
            return abc;
        })
        .join()
        .unwrap();
    }

    fn handle_connection(self, mut stream: TcpStream) -> (Catalog, Storage) {
        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();

        let execute = b"GET /execute";
        let create = b"GET /execute?create";
        let select = b"GET /execute?select";
        let insert = b"GET /execute?insert";
        let exit = b"GET /exit";
        if buffer.starts_with(execute) {
            if buffer.starts_with(create) {
                let res = self.execute_handler(String::from("create"));
                return (res.0, res.1);
            } else if buffer.starts_with(select) {
                let res = self.execute_handler(String::from("select"));
                return (res.0, res.1);
            } else if buffer.starts_with(insert) {
                let res = self.execute_handler(String::from("insert"));
                return (res.0, res.1);
            }
            return (self.db.catalog.clone(), self.db.storage.clone());
        } else if buffer.starts_with(exit) {
            self.clone().exit_handler();
        }

        return (self.db.catalog, self.db.storage);
    }

    fn execute_handler(mut self, query: String) -> (Catalog, Storage) {
        println!("Executed");

        let res = self.db.clone().execute(query);
        self.db.catalog = res.0;
        self.db.storage = res.1;

        println!("Message: {}", res.2);
        return (self.db.catalog.clone(), self.db.storage.clone());
    }

    fn exit_handler(self) {
        println!("Exited");
        self.db.terminate();
        std::process::exit(0);
    }
}

pub fn new_api_server(db: Disakub) -> ApiServer {
    return ApiServer { db };
}
