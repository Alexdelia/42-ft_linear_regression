use estimate::estimate;

use load::Coord;

use crate::{ComputedData, Float};

pub fn learn(data: &ComputedData<Float>, iteration: usize, learning_rate: Float) -> (Float, Float) {
	let mut theta0 = 0.0;
	let mut theta1 = 0.0;

	for _ in 0..iteration {
		(theta0, theta1) = guess(data, theta0, theta1, learning_rate);
	}

	dbg!(theta0, theta1);

	denormalize_theta(theta0, theta1, data)
}

fn denormalize_theta(theta0: Float, theta1: Float, data: &ComputedData<Float>) -> (Float, Float) {
	let range_ratio = data.attr.range.y / data.attr.range.x;

	let theta1 = theta1 * range_ratio;
	let theta0 = theta0 * range_ratio + data.attr.mean.y - theta1 * data.attr.mean.x;

	(theta0, theta1)
}

fn guess(
	data: &ComputedData<Float>,
	theta0: Float,
	theta1: Float,
	learning_rate: Float,
) -> (Float, Float) {
	let mut sum: (Float, Float) = (0.0, 0.0);

	for Coord { x, y } in data.set.normalized.iter() {
		let guess = estimate(theta0, theta1, *x);
		let diff = guess - y;

		sum.0 += diff;
		sum.1 += diff * x;
	}

	(
		theta0 - learning_rate * (sum.0 / data.set.normalized.len() as Float),
		theta1 - learning_rate * (sum.1 / data.set.normalized.len() as Float),
	)
}
