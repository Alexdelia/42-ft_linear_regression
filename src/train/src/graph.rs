use plotters::prelude::*;

use load::Coord;

use crate::{ComputedData, Float};

const OUTPUT_GRAPH: &str = "output/graph.png";
const GRAPH_SIZE: (u32, u32) = (1000, 750);

const REGRESSION_LINE_STYLE: ShapeStyle = ShapeStyle {
	color: RGBAColor(3, 136, 252, 0.42),
	filled: true,
	stroke_width: 2,
};
const DATA_DOT_STYLE: ShapeStyle = ShapeStyle {
	color: RGBAColor(16, 227, 82, 1.0),
	filled: true,
	stroke_width: 1,
};
const DIFF_STYLE: ShapeStyle = ShapeStyle {
	color: RGBAColor(255, 0, 0, 1.0),
	filled: true,
	stroke_width: 1,
};

pub fn graph(data: &ComputedData<Float>, theta0: Float, theta1: Float) -> hmerr::Result<()> {
	std::fs::create_dir_all(
		std::path::Path::new(OUTPUT_GRAPH)
			.parent()
			.unwrap_or_else(|| panic!("{OUTPUT_GRAPH} has no parent")),
	)?;

	let root = BitMapBackend::new(OUTPUT_GRAPH, GRAPH_SIZE).into_drawing_area();

	root.fill(&WHITE)?;

	let graph_coord = compute_graph_coord(data, theta0, theta1);

	let mut chart = ChartBuilder::on(&root)
		.x_label_area_size(40)
		.y_label_area_size(40)
		.caption("Linear Regression", ("Roboto Mono", 20))
		.margin_left(10)
		.margin_right(15)
		.build_cartesian_2d(
			graph_coord.min.x..graph_coord.max.x,
			graph_coord.min.y..graph_coord.max.y,
		)?;

	chart.configure_mesh().draw()?;

	// estimation offset with data set
	for c in data.set.raw.iter() {
		chart.draw_series(LineSeries::new(
			[(c.x, c.y), (c.x, estimate::estimate(theta0, theta1, c.x))],
			DIFF_STYLE,
		))?;
	}

	// regression line
	chart
		.draw_series(LineSeries::new(
			[
				(graph_coord.line.0.x, graph_coord.line.0.y),
				(graph_coord.line.1.x, graph_coord.line.1.y),
			],
			REGRESSION_LINE_STYLE,
		))?
		.label(format!("y = {theta0:.2} + ({theta1:.2} * x)"))
		.legend(|(x, y)| PathElement::new(vec![(x, y), (x + 16, y)], REGRESSION_LINE_STYLE));

	// data set as points
	chart
		.draw_series(
			data.set
				.raw
				.iter()
				.map(|r| Circle::new((r.x, r.y), 3, DATA_DOT_STYLE)),
		)?
		.label(format!(
			"estimation diff sum = {diff:.4}",
			diff = data.diff(theta0, theta1)
		))
		.legend(|(x, y)| PathElement::new(vec![(x, y), (x + 16, y)], DIFF_STYLE));

	chart
		.configure_series_labels()
		.border_style(BLACK)
		.background_style(WHITE.mix(0.8))
		.label_font(TextStyle::from(("Roboto Mono", 16)).color(&RGBColor(42, 42, 42)))
		.draw()?;

	root.present()?;

	Ok(())
}

const SCALE_PERCENTAGE: Float = 0.1;

struct GraphCoord<F> {
	min: Coord<F>,
	max: Coord<F>,
	// center: Coord<F>,
	line: (Coord<F>, Coord<F>),
}

fn compute_graph_coord(
	data: &ComputedData<Float>,
	theta0: Float,
	theta1: Float,
) -> GraphCoord<Float> {
	let min = Coord {
		x: data.attr.min.x,
		y: Float::min(
			data.attr.min.y,
			estimate::estimate(theta0, theta1, data.attr.min.x),
		),
	};
	let max = Coord {
		x: data.attr.max.x,
		y: Float::max(
			data.attr.max.y,
			estimate::estimate(theta0, theta1, data.attr.max.x),
		),
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
			y: estimate::estimate(theta0, theta1, line_min_x),
		},
		Coord {
			x: line_max_x,
			y: estimate::estimate(theta0, theta1, line_max_x),
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
