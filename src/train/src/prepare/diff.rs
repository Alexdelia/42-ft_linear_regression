use std::iter::Sum;
use std::ops::{Add, Mul, Sub};

use estimate::estimate;
use load::Coord;

use crate::ComputedData;

impl<F: Add<Output = F> + Sub<Output = F> + Mul<Output = F> + Sum + Copy> ComputedData<F> {
	pub fn diff(&self, theta0: F, theta1: F) -> F {
		self.set
			.raw
			.iter()
			.map(|Coord { x, y }| estimate(theta0, theta1, *x) - *y)
			.sum()
	}
}
