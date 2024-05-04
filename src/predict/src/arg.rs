use ansi::abbrev::{B, D, I, M, R};
use hmerr::se;

use crate::Float;

pub fn usage(args: &[String]) {
	if args.len() == 2 {
		return;
	}

	let cmd = match args.first() {
		Some(cmd) => cmd,
		None => "cargo run",
	};

	println!("usage: {B}{cmd} {M}<x>{D}\n");
}

pub fn x(args: &[String]) -> hmerr::Result<Float> {
	let Some(arg) = args.get(1) else {
		return Err(se!(
			format!("must specify {B}{M}<x>{D} to estimate {B}{M}<y>{D}"),
			"float",
			format!("{I}nothing{D}"),
		))?;
	};

	Ok(arg.parse::<Float>().map_err(|e| {
		se!(
			format!("could not {B}{R}parse{D} {B}{M}<x>{D}"),
			"float",
			arg,
			s:e,
		)
	})?)
}
