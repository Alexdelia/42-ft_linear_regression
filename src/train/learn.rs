use estimate::estimate;

use crate::{Data, Float, Record};

pub fn learn(data: Data, iteration: usize, learning_rate: Float) -> (Float, Float) {
	let mut data = data;

	normalize(&mut data);

	let mut theta0 = 0.0;
	let mut theta1 = 0.0;

	for i in 0..iteration {
		(theta0, theta1) = guess(&data, theta0, theta1, learning_rate);
	}

	(theta0, theta1)
}

fn normalize(data: &mut Data) {
	let mut min: Record = Record { x: 0.0, y: 0.0 };
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

	for Record { x, y } in data.iter_mut() {
		*x = (*x - min.x) / (max.x - min.x);
		*y = (*y - min.y) / (max.y - min.y);
	}
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
		theta0 - learning_rate * (1.0 / data.len() as Float) * sum.0,
		theta1 - learning_rate * (1.0 / data.len() as Float) * sum.1,
	)
}
