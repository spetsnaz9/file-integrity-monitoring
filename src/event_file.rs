use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;
use std::error::Error;



pub fn write_log(
    path: &PathBuf,
    content: String
) -> Result<(), Box<dyn Error>> {

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(path)?;

    file.write_all(content.as_bytes())?;

    Ok(())
}
