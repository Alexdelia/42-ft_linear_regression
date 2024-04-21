use estimate::estimate;

use crate::{Data, Float, Record};

pub fn learn(data: &Data, iteration: usize, learning_rate: Float) -> (Float, Float) {
	let mut theta0 = 0.0;
	let mut theta1 = 0.0;

	for _ in 0..iteration {
		dbg!(theta0, theta1);
		for Record { x, y } in data {
			eprintln!(
				"{x}km\t{y}€\t{estimate}€",
				x = x,
				y = y,
				estimate = estimate(theta0, theta1, *x)
			);
		}
		(theta0, theta1) = guess(data, theta0, theta1, learning_rate);
	}

	(theta0, theta1)
}

fn guess(data: &Data, theta0: Float, theta1: Float, learning_rate: Float) -> (Float, Float) {
	let mut sum: (Float, Float) = (0.0, 0.0);

	for Record { x, y } in data {
		let guess = estimate(theta0, theta1, *x);
		let diff = guess - y;

		sum.0 += diff;
		sum.1 += diff * x;
	}

	(
		theta0 - learning_rate * (sum.0 / data.len() as Float),
		theta1 - learning_rate * (sum.1 / data.len() as Float),
	)
}
