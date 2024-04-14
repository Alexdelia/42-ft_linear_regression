use std::env;

const DEFAULT_CSV: &str = "./ressource/data.csv";

pub fn csv() -> String {
    if let Some(arg) = env::args().nth(1) {
        arg
    } else {
        println!("no CSV file specified, using default");
        DEFAULT_CSV.to_string()
    }
}
