use std::path::Path;

use indicatif::ProgressIterator;

use estimate::estimate;

use load::Coord;

use crate::graph;
use crate::graph::r#const;
use crate::{ComputedData, Float};

pub fn learn<P: AsRef<Path>>(
	data: &ComputedData<Float>,
	iteration: usize,
	learning_rate: Float,
	path: P,
) -> hmerr::Result<(Float, Float)> {
	let mut theta0 = 0.0;
	let mut theta1 = 0.0;

	let root = graph::training::root(path)?;

	let mut next_frame = 0;
	let mut step = 1;
	for i in (0..iteration).progress() {
		if i == next_frame {
			let (dtheta0, dtheta1) = denormalize_theta(theta0, theta1, data);
			graph::training::graph(&root, &data, dtheta0, dtheta1, i)?;
			next_frame += step;
			if next_frame >= step * r#const::GIF_FRAME_STEP_THRESHOLD {
				step = (step as Float * r#const::GIF_FRAME_STEP_MULTIPLIER).ceil() as usize;
			}
		}

		(theta0, theta1) = guess(data, theta0, theta1, learning_rate);
	}

	Ok(denormalize_theta(theta0, theta1, data))
}

fn denormalize_theta(theta0: Float, theta1: Float, data: &ComputedData<Float>) -> (Float, Float) {
	let theta1 = theta1 * data.attr.range_ratio;
	let theta0 = theta0 * data.attr.range_ratio + data.attr.mean.y - theta1 * data.attr.mean.x;

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
