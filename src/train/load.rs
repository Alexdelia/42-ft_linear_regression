use std::path::Path;

use ansi::abbrev::{B, D, G, M, R};
use csv::StringRecord;
use hmerr::{ioe, pfe, ple, pwe};

use crate::Float;

pub type Data = Vec<Record>;

// pub type Record = (Float, Float);

pub struct Record {
	x: Float,
	y: Float,
}

pub fn parse<P: AsRef<Path> + Clone>(path: P) -> hmerr::Result<Data>
where
	String: From<P>,
{
	let mut ret: Data = Vec::new();

	let mut rdr = csv::Reader::from_path(&path).map_err(|e| {
		if !e.is_io_error() {
			return ioe!(path.clone(), e);
		}
		match e.into_kind() {
			csv::ErrorKind::Io(io_error) => ioe!(path.clone(), io_error),
			_ => unreachable!(),
		}
	})?;

	for (i, result) in rdr.records().enumerate() {
		let record = result.map_err(|e| ioe!(path.clone(), e))?;
		eprintln!("{i}\t{record:?}");

		let record = parse_record(&path, i, record)?;
		eprintln!("x: {x}\ty: {y}", x = record.x, y = record.y);

		ret.push(record);
	}

	Ok(ret)
}

fn parse_record<P: AsRef<Path> + Clone>(
	path: &P,
	i: usize,
	record: StringRecord,
) -> hmerr::Result<Record>
where
	String: From<P>,
{
	let index = i + 2;

	if record.len() < 2 {
		return pfe!(
			"CSV record should have 2 elements\n<x> <y>",
			f:path.clone(),
			l:ple!(string_record_into_line(record), i:index),
		)?;
	}

	Ok(Record {
		x: parse_cell(&path, index, &record, false)?,
		y: parse_cell(&path, index, &record, true)?,
	})
}

fn parse_cell<P: AsRef<Path> + Clone>(
	path: &P,
	index: usize,
	record: &StringRecord,
	y: bool,
) -> hmerr::Result<Float>
where
	String: From<P>,
{
	let cell = record.get(y as usize).unwrap(); // record.len() is being checked above

	match cell.parse::<Float>() {
		Ok(n) => Ok(n),
		Err(e) => {
			let element = format!("{B}{M}<{element}>{D}", element = if y { "y" } else { "x" });

			pfe!(
				format!("failed to {B}{R}parse {element}"),
				h:format!("{element} must be a {B}{G}number{D}"),
				f:path.clone(),
				l:ple!(
					string_record_into_line(record.clone()),
					i:index,
					w:pwe!(cell),
				),
				s:e,
			)?
		}
	}
}

fn string_record_into_line(record: StringRecord) -> String {
	record
		.into_iter()
		.map(|s| s.to_owned())
		.collect::<Vec<String>>()
		.join(",")
}
