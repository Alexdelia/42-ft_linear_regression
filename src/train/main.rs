mod arg;
mod load;

const LEARNING_RATE: f64 = 0.01;

fn estimate(theta0: f64, theta1: f64, x: f64) -> f64 {
	theta0 + (theta1 * x)
}

fn main() -> hmerr::Result<()> {
	println!("Hello, world!");

	let csv = arg::csv();

	load::parse(csv)?;

	Ok(())
}
