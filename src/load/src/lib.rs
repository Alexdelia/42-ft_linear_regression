use std::path::Path;
use std::str::FromStr;

use ansi::abbrev::{B, D, G, M, R};
use csv::StringRecord;
use hmerr::{ioe, pfe, ple, pwe};

pub struct LoadedData<F> {
	pub headers: Coord<String>,
	pub set: ParsedData<F>,
}

pub type ParsedData<F> = Vec<Coord<F>>;

pub struct Coord<F> {
	pub x: F,
	pub y: F,
}

const DEFAULT_X_HEADER: &str = "x";
const DEFAULT_Y_HEADER: &str = "y";

pub fn parse<P: AsRef<Path> + Clone, F: FromStr>(path: P) -> hmerr::Result<LoadedData<F>>
where
	String: From<P>,
	<F as FromStr>::Err: std::error::Error + Sync + Send + 'static,
{
	let mut set: ParsedData<F> = Vec::new();

	let mut rdr = csv::Reader::from_path(&path).map_err(|e| {
		if !e.is_io_error() {
			return ioe!(path.clone(), e);
		}
		match e.into_kind() {
			csv::ErrorKind::Io(io_error) => ioe!(path.clone(), io_error),
			_ => unreachable!(),
		}
	})?;

	let headers: Coord<String> = match rdr.headers() {
		Ok(h) => Coord {
			x: h.get(0).unwrap_or(DEFAULT_X_HEADER).to_string(),
			y: h.get(1).unwrap_or(DEFAULT_Y_HEADER).to_string(),
		},
		Err(_) => Coord {
			x: DEFAULT_X_HEADER.to_string(),
			y: DEFAULT_Y_HEADER.to_string(),
		},
	};

	for (i, result) in rdr.records().enumerate() {
		let record = result.map_err(|e| ioe!(path.clone(), e))?;

		set.push(parse_record(&path, i, record)?);
	}

	if set.is_empty() {
		pfe!(
			"CSV file should contain at least one Coord",
			f:path.clone(),
		)?;
	}

	Ok(LoadedData { headers, set })
}

fn parse_record<P: AsRef<Path> + Clone, F: FromStr>(
	path: &P,
	i: usize,
	record: StringRecord,
) -> hmerr::Result<Coord<F>>
where
	String: From<P>,
	<F as FromStr>::Err: std::error::Error + Sync + Send + 'static,
{
	let index = i + 2;

	#[allow(clippy::needless_return_with_question_mark)]
	if record.len() < 2 {
		return pfe!(
			"CSV Coord should have 2 elements\n<x> <y>",
			f:path.clone(),
			l:ple!(string_record_into_line(record), i:index),
		)?;
	}

	Ok(Coord {
		x: parse_cell(path, index, &record, false)?,
		y: parse_cell(path, index, &record, true)?,
	})
}

fn parse_cell<P: AsRef<Path> + Clone, F: FromStr>(
	path: &P,
	index: usize,
	record: &StringRecord,
	y: bool,
) -> hmerr::Result<F>
where
	String: From<P>,
	<F as FromStr>::Err: std::error::Error + Sync + Send + 'static,
{
	let cell = record.get(y as usize).unwrap(); // Coord.len() is being checked above

	match cell.parse::<F>() {
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
