use std::{error::Error, path::Path};
use hmerr::ioe;

pub type Data = Vec<Record>;

pub type Record = (f64, f64);

pub fn parse<P: AsRef<Path>>(path: P) -> hmerr::Result<Data>
where String: From<P>
{
	let mut ret: Data = Vec::new();

	let mut rdr = csv::Reader::from_path(&path).map_err(|e| {
		if !e.is_io_error() {
			return ioe!(path, e);
		}
		match e.into_kind() {
			csv::ErrorKind::Io(io_error) => ioe!(path, io_error),
			_ => unreachable!()
		}
	})?;

	for result in rdr.deserialize() {
		let record: Record = result?;
		println!("{:?}", record);

		ret.push(record);
	}

	Ok(ret)
}
