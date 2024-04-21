use std::ops::{Add, Mul};

pub fn estimate<F>(theta0: F, theta1: F, x: F) -> F
where
	F: Add<F, Output = F> + Mul<F, Output = F> + Sized,
{
	theta0 + (theta1 * x)
}
