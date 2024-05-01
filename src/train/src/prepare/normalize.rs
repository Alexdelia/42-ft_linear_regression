use load::{Coord, ParsedData};

use crate::Float;

use super::AnalyzedData;

pub fn normalize(data: &ParsedData<Float>, analyzed: &AnalyzedData<Float>) -> ParsedData<Float> {
	let mut normalized = Vec::with_capacity(data.len());

	for Coord { x, y } in data.iter() {
		let x = (x - analyzed.min.x) / (analyzed.range.x);
		let y = (y - analyzed.min.y) / (analyzed.range.y);

		normalized.push(Coord { x, y });
	}

	normalized
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn normalize_basic() {
		let data = vec![
			Coord { x: 1.0, y: 1.0 },
			Coord { x: 2.0, y: 2.0 },
			Coord { x: 3.0, y: 3.0 },
		];

		let analyzed = AnalyzedData {
			min: Coord { x: 1.0, y: 1.0 },
			max: Coord { x: 3.0, y: 3.0 },
			range: Coord { x: 2.0, y: 2.0 },
			range_ratio: 1.0,
			mean: Coord { x: 2.0, y: 2.0 },
		};

		let normalized = normalize(&data, &analyzed);

		assert_eq!(normalized[0].x, 0.0);
		assert_eq!(normalized[0].y, 0.0);

		assert_eq!(normalized[1].x, 0.5);
		assert_eq!(normalized[1].y, 0.5);

		assert_eq!(normalized[2].x, 1.0);
		assert_eq!(normalized[2].y, 1.0);
	}
}
