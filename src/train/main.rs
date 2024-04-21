mod arg;
mod learn;
mod load;

// TODO: better error message for file not found
// TODO: better error for invalid record in csv
// TODO: set learning rate through arg 2

type Float = f64;
pub use load::{Data, Record};

fn main() -> hmerr::Result<()> {
	println!("Hello, world!");

	let csv = arg::csv();

	let data = load::parse(csv)?;

	Ok(())
}
