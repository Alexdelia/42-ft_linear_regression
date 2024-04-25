use estimate::estimate;

use load::Record;

use crate::{ComputedData, Float};

pub fn learn(data: ComputedData<Float>, iteration: usize, learning_rate: Float) -> (Float, Float) {
	let mut data = data;

	normalize(&mut data);

	let mut theta0 = 0.0;
	let mut theta1 = 0.0;

	for _ in 0..iteration {
		(theta0, theta1) = guess(&data, theta0, theta1, learning_rate);
	}

	(theta0, theta1)
}

fn normalize(data: &mut ComputedData<Float>) {
	let mut min = Record { x: 0.0, y: 0.0 };
	let mut max = Record { x: 0.0, y: 0.0 };

	for Record { x, y } in data.set.iter() {
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

	for Record { x, y } in data.set.iter_mut() {
		*x = (*x - min.x) / (max.x - min.x);
		*y = (*y - min.y) / (max.y - min.y);
	}
}

pub fn unnormalize_theta(
	theta0: Float,
	theta1: Float,
	data: &ComputedData<Float>,
) -> (Float, Float) {
	let mut min = Record { x: 0.0, y: 0.0 };
	let mut max = Record { x: 0.0, y: 0.0 };

	for Record { x, y } in data.set.iter() {
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

	(
		theta0 * (max.y - min.y) + min.y,
		theta1 * (max.y - min.y) / (max.x - min.x),
	)
}

fn guess(
	data: &ComputedData<Float>,
	theta0: Float,
	theta1: Float,
	learning_rate: Float,
) -> (Float, Float) {
	let mut sum: (Float, Float) = (0.0, 0.0);

	for Record { x, y } in data.set.iter() {
		let guess = estimate(theta0, theta1, *x);
		let diff = guess - y;

		sum.0 += diff;
		sum.1 += diff * x;
	}

	(
		theta0 - learning_rate * (sum.0 / data.set.len() as Float),
		theta1 - learning_rate * (sum.1 / data.set.len() as Float),
	)
}
