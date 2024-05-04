pub mod r#const;
mod coord;
mod draw;
pub mod result;
pub mod training;

use std::path::Path;

use plotters::{coord::types::RangedCoordf64, prelude::*};

use crate::Float;

use coord::GraphCoord;

pub fn graph_output<P: AsRef<Path>>(path: P, filename: &str) -> hmerr::Result<String> {
	std::fs::create_dir_all(r#const::OUTPUT_GRAPH_DIR)?;

	Ok([r#const::OUTPUT_GRAPH_DIR, filename].join(""))
}

pub fn chart_config<'a, 'b, DB>(
	chart: &mut ChartBuilder<'a, 'b, DB>,
	graph_coord: &GraphCoord<Float>,
	title: &str,
) -> hmerr::Result<ChartContext<'a, DB, Cartesian2d<RangedCoordf64, RangedCoordf64>>>
where
	DB: DrawingBackend,
	<DB as plotters::prelude::DrawingBackend>::ErrorType: 'static,
{
	let mut chart = chart
		.x_label_area_size(40)
		.y_label_area_size(40)
		.caption(title, ("Roboto Mono", 20))
		.margin_left(10)
		.margin_right(15)
		.build_cartesian_2d(
			graph_coord.min.x..graph_coord.max.x,
			graph_coord.min.y..graph_coord.max.y,
		)?;

	chart.configure_mesh().draw()?;

	Ok(chart)
}
