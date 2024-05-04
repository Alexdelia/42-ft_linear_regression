mod arg;
mod prepare;
pub use prepare::ComputedData;
mod learn;

#[cfg(not(debug_assertions))]
mod table;

#[cfg(not(debug_assertions))]
mod graph;

use std::env;

type Float = f64;

const DEFAULT_DATA_FILE: &str = "ressource/42_provided.csv";
const DEFAULT_ITERATION: usize = 2usize.pow(16);
const DEFAULT_LEARNING_RATE: Float = 0.01;

fn main() -> hmerr::Result<()> {
	let args = env::args().collect::<Vec<String>>();

	arg::usage(&args);

	let csv = arg::csv(&args);
	let iteration = arg::iteration(&args)?;
	let learning_rate = arg::learning_rate(&args)?;

	println!();

	let data = load::parse(csv)?;

	let data = prepare::compute(data);

	let (theta0, theta1) = learn::learn(csv, &data, iteration, learning_rate)?;

	#[cfg(not(debug_assertions))]
	table::table(&data, theta0, theta1);

	#[cfg(not(debug_assertions))]
	graph::result::graph(csv, &data, theta0, theta1)?;

	model::Model { theta0, theta1 }.write()?;

	Ok(())
}
