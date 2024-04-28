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

	for Coord { x, y } in data.iter().skip(1) {
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

	AnalyzedData { min, max }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn analyze_basic() {
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
	}

	#[test]
	fn analyze_above_0() {
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
	}
}
