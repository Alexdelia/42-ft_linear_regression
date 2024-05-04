pub mod r#const;
mod coord;
mod draw;
pub mod result;
pub mod training;

use std::path::Path;

use plotters::{coord::types::RangedCoordf64, prelude::*};

use load::Coord;

use crate::Float;

use coord::GraphCoord;

pub fn graph_output<P: AsRef<Path>>(path: P, filename: &str) -> hmerr::Result<String> {
	let path = path
		.as_ref()
		.file_stem()
		.expect("there is no path")
		.to_str()
		.expect("path is not a valid UTF-8 string");

	let dir = format!("{graph}{path}/", graph = r#const::OUTPUT_GRAPH_DIR,);

	std::fs::create_dir_all(&dir)?;

	Ok([dir.as_str(), filename].join(""))
}

pub fn chart_config<'a, DB>(
	chart: &mut ChartBuilder<'a, '_, DB>,
	graph_coord: &GraphCoord<Float>,
	title: &str,
	headers: &Coord<String>,
) -> hmerr::Result<ChartContext<'a, DB, Cartesian2d<RangedCoordf64, RangedCoordf64>>>
where
	DB: DrawingBackend,
	<DB as plotters::prelude::DrawingBackend>::ErrorType: 'static,
{
	let mut chart = chart
		.x_label_area_size(30)
		.y_label_area_size(60)
		.caption(title, ("Roboto Mono", 20))
		.margin_left(5)
		.margin_bottom(5)
		.margin_right(15)
		.build_cartesian_2d(
			graph_coord.min.x..graph_coord.max.x,
			graph_coord.min.y..graph_coord.max.y,
		)?;

	chart
		.configure_mesh()
		.x_desc(&headers.x)
		.y_desc(&headers.y)
		.draw()?;

	Ok(chart)
}
