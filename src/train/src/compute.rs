use load::{ParsedData, Record};

use crate::Float;

pub struct ComputedData<F> {
	pub set: ParsedData<F>,
	pub min: Record<F>,
	pub max: Record<F>,
}

pub fn compute(data: ParsedData<Float>) -> ComputedData<Float> {
	let mut min = Record { x: 0.0, y: 0.0 };
	let mut max = Record { x: 0.0, y: 0.0 };

	for Record { x, y } in data.iter() {
		if *x < min.x {
			min.x = *x;
		} else if *x > max.x {
			max.x = *x;
		}

		if *y < min.y {
			min.y = *y;
		} else if *y > max.y {
			max.y = *y;
		}
	}

	ComputedData {
		set: data,
		min,
		max,
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_compute() {
		let data = vec![
			Record { x: 42.0, y: -69.0 },
			Record {
				x: -21.0,
				y: 2048.0,
			},
			Record { x: 84.0, y: 0.0 },
		];

		let computed = compute(data);

		assert_eq!(computed.min.x, -21.0);
		assert_eq!(computed.min.y, -69.0);
		assert_eq!(computed.max.x, 84.0);
		assert_eq!(computed.max.y, 2048.0);
	}
}
