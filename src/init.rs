extern crate serde;
extern crate serde_json;
extern crate serde_derive;

use std::path::Path;
use std::error::Error;
use std::fs::{self, File};
use sha2::{Digest, Sha256};



struct PathJson {
    file: File,
    list: Vec<ContentJson>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContentJson {
    path: String,
    hash: String,
}

impl PathJson {
    fn new() -> Result<PathJson, Box<dyn Error>> {
        let json_path = Path::new("save/path.json");
        let file = File::open(json_path)?;

        return Ok(PathJson {
            file,
            list: Vec::new(),
        });
    }

    fn read(
        &mut self,
    ) -> Result<(), Box<dyn Error>> {

        self.list = serde_json::from_reader(&self.file)?;

        Ok(())
    }
}

pub fn init(
    dir: &Path
) -> Result<(), Box<dyn Error>> {

    let mut path_json = PathJson::new()?;
    path_json.read()?;
    println!("{:?}", path_json.list);

    rec_check(dir)?;

    Ok(())
}

fn rec_check(
    dir: &Path,
) -> Result<(), Box<dyn Error>> {
   
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                println!("fichier : {:?}", path);
            } else if path.is_dir() {
                println!("dossier : {:?}", path);
                rec_check(&path)?;
            }
        }
    }

    Ok(())
}

fn sha256_string(
    input: &str,
) -> String {

    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    let hex_string = result.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();
    hex_string
}
