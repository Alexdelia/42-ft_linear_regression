mod analyze;
mod diff;
mod normalize;

use load::{Coord, LoadedData, ParsedData};

use crate::Float;

pub struct ComputedData<F> {
	pub headers: Coord<String>,
	pub set: DataSet<F>,
	pub attr: AnalyzedData<F>,
}

pub struct DataSet<F> {
	pub raw: ParsedData<F>,
	pub normalized: ParsedData<F>,
}

pub struct AnalyzedData<F> {
	pub min: Coord<F>,
	pub max: Coord<F>,
	pub range: Coord<F>,
	pub range_ratio: F,
	pub mean: Coord<F>,
}

pub fn compute(data: LoadedData<Float>) -> ComputedData<Float> {
	let analyzed = analyze::analyze(&data.set);

	let normalized = normalize::normalize(&data.set, &analyzed);

	ComputedData {
		headers: data.headers,
		set: DataSet {
			raw: data.set,
			normalized,
		},
		attr: analyzed,
	}
}
