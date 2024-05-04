use ansi::abbrev::{B, Y, R, D};

type Float = f64;

fn main() -> hmerr::Result<()> {
	println!("Hello, world!");
	let model = match model::Model::<Float>::read() {
		Ok(model) => Ok(model),
		Err(e) => {
			eprintln!("{B}{Y}make sure to {R}train {Y}the model first{D}");
			Err(e)
		}
	}?;

	dbg!(model.theta0, model.theta1);

	Ok(())
}
