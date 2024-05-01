use estimate::estimate;
use load::Coord;

use crate::{ComputedData, Float};

impl ComputedData<Float> {
	pub fn diff(&self, theta0: Float, theta1: Float) -> Float {
		self.set
			.raw
			.iter()
			.map(|Coord { x, y }| (estimate(theta0, theta1, *x) - *y).abs())
			.sum()
	}
}
