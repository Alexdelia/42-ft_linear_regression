use estimate::estimate;
use load::Coord;

use crate::{ComputedData, Float};

const SCALE_PERCENTAGE: Float = 0.1;

pub struct GraphCoord<F> {
	pub min: Coord<F>,
	pub max: Coord<F>,
	// pub center: Coord<F>,
	pub line: (Coord<F>, Coord<F>),
}

pub fn compute_graph_coord(
	data: &ComputedData<Float>,
	theta0: Float,
	theta1: Float,
) -> GraphCoord<Float> {
	let min = Coord {
		x: data.attr.min.x,
		y: Float::min(data.attr.min.y, estimate(theta0, theta1, data.attr.min.x)),
	};
	let max = Coord {
		x: data.attr.max.x,
		y: Float::max(data.attr.max.y, estimate(theta0, theta1, data.attr.max.x)),
	};
	let center = Coord {
		x: (min.x + max.x) / 2.0,
		y: (min.y + max.y) / 2.0,
	};

	let scale = |v: Float, center: Float, high: bool| {
		v + ((center * SCALE_PERCENTAGE).abs() * if high { 1.0 } else { -1.0 })
	};

	let line_min_x = scale(data.attr.min.x, center.x / 2.0, false);
	let line_max_x = scale(data.attr.max.x, center.x / 2.0, true);
	let line = (
		Coord {
			x: line_min_x,
			y: estimate(theta0, theta1, line_min_x),
		},
		Coord {
			x: line_max_x,
			y: estimate(theta0, theta1, line_max_x),
		},
	);

	let min = Coord {
		x: scale(min.x, center.x, false),
		y: scale(min.y, center.y, false),
	};
	let max = Coord {
		x: scale(max.x, center.x, true),
		y: scale(max.y, center.y, true),
	};

	GraphCoord {
		min,
		max,
		// center,
		line,
	}
}
