mod arg;
mod compute;
pub use compute::ComputedData;
mod graph;
mod learn;

use load::Coord;
use std::env;

use crate::compute::compute;

type Float = f64;

const DEFAULT_DATA_FILE: &str = "ressource/data.csv";
const DEFAULT_ITERATION: usize = 100000;
const DEFAULT_LEARNING_RATE: Float = 0.01;

fn main() -> hmerr::Result<()> {
	let args = env::args().collect::<Vec<String>>();

	arg::usage(&args);

	let csv = arg::csv(&args);
	let iteration = arg::iteration(&args)?;
	let learning_rate = arg::learning_rate(&args)?;

	let data = load::parse(csv)?;

	let data = compute(data);

	let (theta0, theta1) = learn::learn(&data, iteration, learning_rate);

	let mut diff_sum_abs = 0.0;
	let mut diff_sum = 0.0;

	for Coord { x, y } in data.set.raw.iter() {
		eprintln!(
			"{x}km\n= {y}€\n~ {estimate}€\n",
			x = x,
			y = y,
			estimate = estimate::estimate(theta0, theta1, *x),
		);

		diff_sum_abs += (estimate::estimate(theta0, theta1, *x) - y).abs();
		diff_sum += estimate::estimate(theta0, theta1, *x) - y;
	}

	dbg!(theta0, theta1);
	dbg!(diff_sum_abs, diff_sum);

	graph::graph(&data, theta0, theta1)?;

	Ok(())
}
