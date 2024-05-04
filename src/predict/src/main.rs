mod arg;

use std::env;

use ansi::abbrev::{B, D, R, Y};

use estimate::estimate;
use model::Model;

type Float = f64;

fn main() -> hmerr::Result<()> {
	let Model { theta0, theta1 } = match Model::<Float>::read() {
		Ok(model) => Ok(model),
		Err(e) => {
			eprintln!("{B}{Y}make sure to {R}train {Y}the model first{D}");
			Err(e)
		}
	}?;

	let args = env::args().collect::<Vec<String>>();

	arg::usage(&args);

	let x = arg::x(&args)?;

	println!("{y}", y=estimate(theta0, theta1, x));

	Ok(())
}
