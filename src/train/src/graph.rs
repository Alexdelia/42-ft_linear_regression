use plotters::prelude::*;

use crate::{ComputedData, Float, OUTPUT_GRAPH};

pub fn graph(data: &ComputedData<Float>, theta0: Float, theta1: Float) -> hmerr::Result<()> {
	let root = BitMapBackend::new(OUTPUT_GRAPH, (800, 600)).into_drawing_area();

	root.fill(&WHITE)?;

	let mut chart = ChartBuilder::on(&root)
		.x_label_area_size(40)
		.y_label_area_size(40)
		.margin(10)
		.build_cartesian_2d(
			(data.min.x * 1.1)..(data.max.x * 1.1),
			(data.min.y * 1.1)..(data.max.y * 1.1),
		)?;

	chart.configure_mesh().draw()?;

	let coord = data.set.iter().map(|r| (r.x, r.y));

	chart.draw_series(LineSeries::new(coord, &BLACK).point_size(5))?;

	let x = data.set.iter().map(|r| r.x);
	let y = data
		.set
		.iter()
		.map(|r| estimate::estimate(theta0, theta1, r.x));

	chart.draw_series(LineSeries::new(x.clone().zip(y.clone()), &RED).point_size(10))?;

	root.present()?;

	Ok(())
}
