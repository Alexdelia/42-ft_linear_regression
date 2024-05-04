use tabled::{
	builder::Builder,
	settings::{object::Rows, style::BorderColor, themes::Colorization, Alignment, Color, Style},
};

use estimate::estimate;

use crate::{ComputedData, Float};

pub fn table(data: &ComputedData<Float>, theta0: Float, theta1: Float) -> () {
	let mut builder = Builder::default();

	builder.push_record([
		&data.headers.x,
		&data.headers.y,
		&format!("estimated {y}", y = data.headers.y),
		"diff",
	]);

	for c in &data.set.raw {
		let ey = estimate(theta0, theta1, c.x);

		builder.push_record([c.x, c.y, ey, c.y - ey].map(|n| format!("{n:.2}")));
	}

	let table = builder
		.build()
		.with(Style::rounded())
		.with(BorderColor::filled(Color::new("\u{1b}[2;35m", "\u{1b}[0m")))
		.with(Colorization::exact(
			[Color::new("\u{1b}[1m", "\u{1b}[0m")],
			Rows::first(),
		))
		.with(Colorization::exact(
			[Color::new("\u{1b}[1;36m", "\u{1b}[0m")],
			(0, 2),
		))
		.with(Colorization::exact(
			[Color::new("\u{1b}[1;31m", "\u{1b}[0m")],
			(0, 3),
		))
		.modify(Rows::new(1..), Alignment::right())
		.to_string();

	println!("{table}")
}
