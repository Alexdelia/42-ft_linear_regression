use std::{fs::File, io::Write};

use serde::{Deserialize, Serialize};

use hmerr::ioe;

const MODEL_PATH: &str = "model.toml";

#[derive(Serialize, Deserialize)]
pub struct Model<F> {
	pub theta0: F,
	pub theta1: F,
}

impl<F: Serialize> Model<F> {
	pub fn write(self) -> hmerr::Result<()> {
		let mut file = File::create(MODEL_PATH).map_err(|e| ioe!(MODEL_PATH, e))?;

		let str_data = toml::to_string(&self)?;

		file.write_all(str_data.as_bytes())
			.map_err(|e| ioe!(MODEL_PATH, e))?;

		Ok(())
	}
}

impl<F: for<'de> Deserialize<'de>> Model<F> {
	pub fn read() -> hmerr::Result<Self> {
		let str_data = std::fs::read_to_string(MODEL_PATH).map_err(|e| ioe!(MODEL_PATH, e))?;

		Ok(toml::from_str(&str_data)?)
	}
}
