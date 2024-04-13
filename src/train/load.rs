use std::{error::Error, path::Path};

pub type Data = Vec<Record>;

pub type Record = (f64, f64);

pub fn run<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
