use std::{
    io::Read,
    net::{TcpListener, TcpStream},
    thread,
};

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
            let mut cloned_api_server = cloned_self.clone();
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                cloned_api_server = thread::spawn({
                    let mut api_server = cloned_api_server.clone();
                    move || {
                        api_server.db = api_server.clone().handle_connection(stream);
                        return api_server;
                    }
                })
                .join()
                .unwrap();
            }
            return cloned_api_server;
        })
        .join()
        .unwrap();
    }

    fn handle_connection(self, mut stream: TcpStream) -> Disakub {
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
                return res;
            } else if buffer.starts_with(select) {
                let res = self.execute_handler(String::from("select"));
                return res;
            } else if buffer.starts_with(insert) {
                let res = self.execute_handler(String::from("insert"));
                return res;
            }
            return self.db;
        } else if buffer.starts_with(exit) {
            self.clone().exit_handler();
        }

        return self.db;
    }

    fn execute_handler(mut self, query: String) -> Disakub {
        println!("Executed");

        let res = self.db.clone().execute(query);
        self.db = res.0;

        println!("Message: {}", res.1);
        return self.db;
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
