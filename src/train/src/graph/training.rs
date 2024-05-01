use plotters::{coord::Shift, prelude::*};

use crate::{ComputedData, Float};

use super::{chart_config, coord::compute_graph_coord, draw, graph_output, r#const::*};

pub fn root() -> hmerr::Result<DrawingArea<BitMapBackend<'static>, Shift>> {
	let output = graph_output(OUTPUT_TRAINING_GRAPH)?;

	Ok(BitMapBackend::gif(&output, TRAINING_GRAPH_SIZE, GIF_FRAME_DELAY)?.into_drawing_area())
}

pub fn graph<'a, DB>(
	root: &'a DrawingArea<DB, Shift>,
	data: &ComputedData<Float>,
	theta0: Float,
	theta1: Float,
	iteration: usize,
) -> hmerr::Result<()>
where
	DB: DrawingBackend,
	<DB as plotters::prelude::DrawingBackend>::ErrorType: 'static,
{
	let graph_coord = compute_graph_coord(data, theta0, theta1);

	root.fill(&WHITE)?;

	let mut chart = ChartBuilder::on(root);
	let mut chart = chart_config(
		&mut chart,
		&graph_coord,
		&format!("{TRAINING_TITLE} ({iteration})"),
	)?;

	draw::graph(&mut chart, graph_coord, data, theta0, theta1)?;

	root.present()?;

	Ok(())
}
