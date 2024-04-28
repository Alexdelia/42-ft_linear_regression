mod analyze;
mod normalize;

use load::{Coord, ParsedData};

use crate::Float;

pub struct ComputedData<F> {
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
}

pub fn compute(data: ParsedData<Float>) -> ComputedData<Float> {
	let analyzed = analyze::analyze(&data);

	let normalized = normalize::normalize(&data, &analyzed);

	ComputedData {
		set: DataSet {
			raw: data,
			normalized,
		},
		attr: analyzed,
	}
}
