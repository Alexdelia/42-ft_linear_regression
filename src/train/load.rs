use std::{error::Error, path::Path};

pub type Data = Vec<Record>;

pub type Record = (f64, f64);

pub fn parse<P: AsRef<Path>>(path: P) -> hmerr::Result<Data> {
	let mut ret: Data = Vec::new();

	let mut rdr = csv::Reader::from_path(path).map_err(|e| hmerr::Error))

	for result in rdr.deserialize() {
		let record: Record = result?;
		println!("{:?}", record);

		ret.push(record);
	}

	Ok(ret)
}
