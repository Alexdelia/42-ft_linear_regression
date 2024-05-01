use load::{Coord, ParsedData};

use crate::Float;

use super::AnalyzedData;

pub fn analyze(data: &ParsedData<Float>) -> AnalyzedData<Float> {
	let first = data.first().unwrap_or(&Coord { x: 0.0, y: 0.0 });

	let mut min = Coord {
		x: first.x,
		y: first.y,
	};
	let mut max = Coord {
		x: first.x,
		y: first.y,
	};

	let mut sum = Coord {
		x: first.x,
		y: first.y,
	};

	for Coord { x, y } in data.iter().skip(1) {
		sum.x += x;
		sum.y += y;

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

	let range = Coord {
		x: max.x - min.x,
		y: max.y - min.y,
	};

	let mean = Coord {
		x: sum.x / data.len() as Float,
		y: sum.y / data.len() as Float,
	};

	AnalyzedData {
		min,
		max,
		range,
		mean,
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn analyze_min_max_range_basic() {
		let data = vec![
			Coord { x: 42.0, y: -69.0 },
			Coord {
				x: -21.0,
				y: 2048.0,
			},
			Coord { x: 84.0, y: 0.0 },
		];

		let a = analyze(&data);

		assert_eq!(a.min.x, -21.0);
		assert_eq!(a.min.y, -69.0);
		assert_eq!(a.max.x, 84.0);
		assert_eq!(a.max.y, 2048.0);

		assert_eq!(a.range.x, 105.0);
		assert_eq!(a.range.y, 2117.0);
	}

	#[test]
	fn analyze_min_max_above_0() {
		let data = vec![
			Coord { x: 42.0, y: 69.0 },
			Coord { x: 21.0, y: 2048.0 },
			Coord { x: 84.0, y: 90.0 },
		];

		let a = analyze(&data);

		assert_eq!(a.min.x, 21.0);
		assert_eq!(a.min.y, 69.0);
		assert_eq!(a.max.x, 84.0);
		assert_eq!(a.max.y, 2048.0);

		assert_eq!(a.range.x, 63.0);
		assert_eq!(a.range.y, 1979.0);
	}

	#[test]
	fn analyze_mean_basic() {
		let data = vec![
			Coord { x: 42.0, y: -68.0 },
			Coord {
				x: -21.0,
				y: 2048.0,
			},
			Coord { x: 84.0, y: 0.0 },
		];

		let a = analyze(&data);

		assert_eq!(a.mean.x, 35.0);
		assert_eq!(a.mean.y, 660.0);
	}
}
