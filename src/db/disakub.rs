pub use crate::query::token::*;
pub use crate::storage::catalog::*;
use crate::{
    query::{self},
    storage::{self, storage::Storage},
};

use ctrlc;

#[derive(Clone, Debug)]
pub struct Disakub {
    pub catalog: Catalog,
    pub storage: Storage,
    home: String,
}

impl Disakub {
    pub fn init(self) {
        ctrlc::set_handler(move || {
            println!("received Ctrl+C!");
            std::process::exit(0)
        })
        .expect("Error setting Ctrl-C handler");
    }

    pub fn execute(mut self, query: String) -> (Disakub, String) {
        let tokenizer = query::token::new_tokenizer(query.clone());
        let tokens = tokenizer.tokenize();

        let parser = query::parse::new_parser(tokens);
        let node = parser.parse();

        let analyzer = query::analyze::new_analyzer(self.catalog.clone());
        let analyzed_query = analyzer.analyze_main(node);

        let planner = query::plan::new_planner(analyzed_query.clone());
        let plan = planner.plan_main();

        let executor = query::executor::new_executor(self.storage, self.catalog);
        let result = executor.execute_main(analyzed_query, plan);
        println!("Query -> {}", query);
        self.catalog = result.0;
        self.storage = result.1;
        return (self, result.2.message);
    }

    pub fn terminate(self) {
        save_catalog(self.home, self.catalog)
    }
}

pub fn new_disakub() -> Disakub {
    let home = match std::env::var("DISAKUB_HOME") {
        Ok(val) => val.to_owned(),
        Err(_e) => "/Users/".to_owned(),
    };
    let catalog = match storage::catalog::load_catalog(home.clone()) {
        Ok(c) => c,
        Err(_) => Catalog {
            schemas: Vec::new(),
        },
    };
    return Disakub {
        catalog,
        storage: storage::storage::new_storage(home.clone()),
        home: home.clone(),
    };
}
