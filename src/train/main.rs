mod arg;
mod learn;
mod load;

pub use load::{Data, Record};

use std::env;

// TODO: better error message for file not found
// TODO: better error for invalid record in csv
// TODO: set learning rate through arg 2

type Float = f64;

const DEFAULT_DATA_FILE: &str = "ressource/data.csv";
const DEFAULT_ITERATION: usize = 100000;
const DEFAULT_LEARNING_RATE: Float = 0.0001;

fn main() -> hmerr::Result<()> {
	let args = env::args().collect::<Vec<String>>();

	arg::usage(&args);

	let csv = arg::csv(&args);
	let iteration = arg::iteration(&args)?;
	let learning_rate = arg::learning_rate(&args)?;

	let data = load::parse(csv)?;

	let (theta0, theta1) = learn::learn(&data, iteration, learning_rate);

	dbg!(theta0, theta1);

	Ok(())
}
