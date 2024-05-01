use plotters::prelude::*;

use crate::{ComputedData, Float};

use super::{chart_config, coord::compute_graph_coord, draw, graph_output, r#const::*};

pub fn graph(data: &ComputedData<Float>, theta0: Float, theta1: Float) -> hmerr::Result<()> {
	let output = graph_output(OUTPUT_RESULT_GRAPH)?;

	let root = BitMapBackend::new(&output, GRAPH_SIZE).into_drawing_area();

	root.fill(&WHITE)?;

	let graph_coord = compute_graph_coord(data, theta0, theta1);

	let mut chart = ChartBuilder::on(&root);
	let mut chart = chart_config(&mut chart, &graph_coord, RESULT_TITLE)?;

	draw::graph(&mut chart, graph_coord, data, theta0, theta1)?;

	root.present()?;

	Ok(())
}
