use estimate::estimate;
use plotters::{coord::types::RangedCoordf64, prelude::*};

use crate::{ComputedData, Float};

use super::{coord::GraphCoord, r#const::*};

pub fn graph<'a, 'b, DB>(
	chart: &mut ChartContext<'a, DB, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
	graph_coord: GraphCoord<Float>,
	data: &ComputedData<Float>,
	theta0: Float,
	theta1: Float,
) -> hmerr::Result<()>
where
	DB: DrawingBackend + 'a,
	<DB as plotters::prelude::DrawingBackend>::ErrorType: 'static,
{
	estimation_offset(chart, data, theta0, theta1)?;
	regression_line(chart, &graph_coord, theta0, theta1)?;
	data_points(chart, data, theta0, theta1)?;

	configure_labels(chart)?;

	Ok(())
}

fn estimation_offset<DB>(
	chart: &mut ChartContext<'_, DB, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
	data: &ComputedData<Float>,
	theta0: Float,
	theta1: Float,
) -> hmerr::Result<()>
where
	DB: DrawingBackend,
	<DB as plotters::prelude::DrawingBackend>::ErrorType: 'static,
{
	for c in data.set.raw.iter() {
		chart.draw_series(LineSeries::new(
			[(c.x, c.y), (c.x, estimate(theta0, theta1, c.x))],
			DIFF_STYLE,
		))?;
	}

	Ok(())
}

fn regression_line<DB>(
	chart: &mut ChartContext<'_, DB, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
	graph_coord: &GraphCoord<Float>,
	theta0: Float,
	theta1: Float,
) -> hmerr::Result<()>
where
	DB: DrawingBackend,
	<DB as plotters::prelude::DrawingBackend>::ErrorType: 'static,
{
	chart
		.draw_series(LineSeries::new(
			[
				(graph_coord.line.0.x, graph_coord.line.0.y),
				(graph_coord.line.1.x, graph_coord.line.1.y),
			],
			REGRESSION_LINE_STYLE,
		))?
		.label(format!("y = {theta0:.2} + ({theta1:.5} * x)"))
		.legend(|(x, y)| PathElement::new(vec![(x, y), (x + 16, y)], REGRESSION_LINE_STYLE));

	Ok(())
}

fn data_points<DB>(
	chart: &mut ChartContext<'_, DB, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
	data: &ComputedData<Float>,
	theta0: Float,
	theta1: Float,
) -> hmerr::Result<()>
where
	DB: DrawingBackend,
	<DB as plotters::prelude::DrawingBackend>::ErrorType: 'static,
{
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

	Ok(())
}

fn configure_labels<'a, 'b, DB>(
	chart: &mut ChartContext<'a, DB, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
) -> hmerr::Result<()>
where
	DB: DrawingBackend + 'a,
	<DB as plotters::prelude::DrawingBackend>::ErrorType: 'static,
{
	chart
		.configure_series_labels()
		.border_style(BLACK)
		.background_style(WHITE.mix(0.8))
		.label_font(TextStyle::from(("Roboto Mono", 16)).color(&RGBColor(42, 42, 42)))
		.draw()?;

	Ok(())
}
