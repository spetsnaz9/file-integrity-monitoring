extern crate serde;
extern crate serde_json;
extern crate serde_derive;

use std::path::{Path, PathBuf};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use sha2::{Digest, Sha256};
use std::collections::HashSet;

use crate::event_file::check_rec;



pub struct PathJson {
    pub path: PathBuf,
    pub file_use: bool,
    pub list: Vec<ContentJson>,
    pub exist: HashSet<PathBuf>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentJson {
    pub path: String,
    pub hash: String,
}

impl PathJson {
    fn new() -> Result<PathJson, Box<dyn Error>> {

        let path = Path::new("save/path.json").to_path_buf();

        return Ok(PathJson {
            path,
            file_use: false,
            list: Vec::new(),
            exist: HashSet::new(),
        });
    }

    fn read(
        &mut self,
    ) -> Result<(), Box<dyn Error>> {

        self.file_use = true;

        let file = File::open(self.path.clone())?;
        self.list = serde_json::from_reader(file)?;

        for content in self.list.iter() {
            let path = Path::new(&content.path).to_path_buf();
            self.exist.insert(path);
        }

        self.file_use = false;
        Ok(())
    }

    pub fn write(
        &mut self
    ) -> Result<(), Box<dyn Error>> {

        self.file_use = true;
        
        let json_string = serde_json::to_string_pretty(&self.list)?;
        let mut file = File::create(self.path.clone())?;

        file.write_all(json_string.as_bytes())?;

        self.file_use = false;
        Ok(())
    }
}

pub fn init(
    dir: &Path
) -> Result<PathJson, Box<dyn Error>> {

    let path = dir.to_path_buf();
    let mut path_json = PathJson::new()?;
    path_json.read()?;

    check_rec(&path, &mut path_json)?;

    Ok(path_json)
}

pub fn sha256_hash(
    path: &Path,
) -> String {

    let path = path.to_string_lossy().to_string();

    let mut hasher = Sha256::new();
    hasher.update(path.as_bytes());
    let result = hasher.finalize();
    
    let hex_string = result
        .iter().
        map(
            |byte| format!("{:02x}", byte)
        ).collect::<String>();
    
    hex_string
}
