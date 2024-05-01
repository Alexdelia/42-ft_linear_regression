use ansi::abbrev::{B, D, M, R};
use hmerr::se;

use crate::{Float, DEFAULT_DATA_FILE, DEFAULT_ITERATION, DEFAULT_LEARNING_RATE};

pub fn usage(args: &[String]) {
	if args.len() > 1 {
		return;
	}

	let cmd = match args.first() {
		Some(cmd) => cmd,
		None => "cargo run",
	};

	println!("usage: {B}{cmd} {M}<CSV> <iteration> <learning rate>{D}\n");
}

pub fn csv(args: &[String]) -> &str {
	if let Some(arg) = args.get(1) {
		arg
	} else {
		println!(
			"no {B}{M}<CSV>{D} file specified, using default file {B}{M}{DEFAULT_DATA_FILE}{D}"
		);
		DEFAULT_DATA_FILE
	}
}

pub fn iteration(args: &[String]) -> hmerr::Result<usize> {
	Ok(if let Some(arg) = args.get(2) {
		arg.parse::<usize>().map_err(|e| {
			se!(
				format!("could not {B}{R}parse{D} {B}{M}<iteration>{D}"),
				"usize",
				arg,
				s:e,
			)
		})?
	} else {
		println!("no {B}{M}<iteration>{D} specified, using default iteration {B}{M}{DEFAULT_ITERATION}{D}");
		DEFAULT_ITERATION
	})
}

pub fn learning_rate(args: &[String]) -> hmerr::Result<Float> {
	Ok(if let Some(arg) = args.get(3) {
		arg.parse::<Float>().map_err(|e| {
			se!(
				format!("could not {B}{R}parse{D} {B}{M}<learning rate>{D}"),
				"float",
				arg,
				s:e,
			)
		})?
	} else {
		println!(
			"no {B}{M}<learning rate>{D} specified, using default learning rate {B}{M}{DEFAULT_LEARNING_RATE}{D}"
		);
		DEFAULT_LEARNING_RATE
	})
}
