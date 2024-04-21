mod arg;
mod learn;
mod load;

pub use load::{Data, Record};

use std::env;

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

	let mut min = Record { x: 0.0, y: 0.0 };
	let mut max = Record { x: 0.0, y: 0.0 };

	for Record { x, y } in data.iter() {
		if *x < min.x {
			min.x = *x;
		} else {
			max.x = *x;
		}

		if *y < min.y {
			min.y = *y;
		} else {
			max.y = *y;
		}
	}

	let (theta0, theta1) = learn::learn(data.clone(), iteration, learning_rate);

	for Record { x, y } in data {
		eprintln!(
			"{x}km\t{y}€\t{estimate}€\t{normilized_estimate_x}€\t{normilized_estimate}€",
			x = x,
			y = y,
			estimate = estimate::estimate(theta0, theta1, x),
			normilized_estimate_x = estimate::normilized_estimate(theta0, theta1, x, min.x, max.x),
			normilized_estimate = estimate::normilized_estimate(theta0, theta1, x, min.x, max.x)
				* (max.y - min.y)
				+ min.y,
		);
	}

	dbg!(theta0, theta1);

	Ok(())
}
