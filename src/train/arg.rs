use ansi::abbrev::{B, D, G, M};
use std::env;

const DEFAULT_CSV: &str = "./ressource/data.csv";

pub fn csv() -> String {
	if let Some(arg) = env::args().nth(1) {
		arg
	} else {
		println!("no {B}{G}CSV{D} file specified, using default file {B}{M}{DEFAULT_CSV}{D}");
		DEFAULT_CSV.to_string()
	}
}
