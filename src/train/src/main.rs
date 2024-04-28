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

	let (theta0, theta1) = learn::learn(data, iteration, learning_rate);

	let data = load::parse::<_, Float>(csv)?;
	let data = compute(data);
	let (utheta0, utheta1) = learn::unnormalize_theta(theta0, theta1, &data);

	eprintln!(
		"theta0:\t{theta0}\ttheta1:\t{theta1}",
		theta0 = theta0,
		theta1 = theta1
	);
	eprintln!(
		"utheta0:\t{utheta0}\tutheta1:\t{utheta1}",
		utheta0 = utheta0,
		utheta1 = utheta1
	);

	let data = load::parse::<_, Float>(csv)?;

	for Coord { x, y } in data.iter() {
		eprintln!(
			"{x}km\n{y}€\n{estimate}€\n{normilized_estimate}€\n",
			x = x,
			y = y,
			estimate = estimate::estimate(theta0, theta1, *x),
			normilized_estimate = estimate::estimate(utheta0, utheta1, *x),
		);
	}

	dbg!(utheta0, utheta1);

	let data = compute(data);

	graph::graph(&data, utheta0, utheta1)?;

	Ok(())
}
