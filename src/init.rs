extern crate serde;
extern crate serde_json;
extern crate serde_derive;

use std::path::{Path, PathBuf};
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use sha2::{Digest, Sha256};
use std::collections::HashSet;



struct PathJson {
    path: PathBuf,
    file_use: bool,
    list: Vec<ContentJson>,
    exist: HashSet<PathBuf>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContentJson {
    path: String,
    hash: String,
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

    fn write(
        &mut self
    ) -> Result<(), Box<dyn Error>> {

        self.file_use = true;
        
        let json_string = serde_json::to_string_pretty(&self.list)?;
        let mut file = File::create(self.path.clone())?;

        file.write_all(json_string.as_bytes())?;

        self.file_use = false;
        Ok(())
    }

    fn check_file(
        &mut self,
        path: &Path,
    ) -> Result<(), Box<dyn Error>> {

        if !self.exist.contains(path) {
            let hash = sha256_hash(path);

            let new = ContentJson {
                path: path.to_string_lossy().to_string(),
                hash,
            };

            self.list.push(new);            
            self.exist.insert(path.to_path_buf());

            self.write()?;
        }
        // else : regarder les modifications pour ce fichier.

        Ok(())
    }
}

pub fn init(
    dir: &Path
) -> Result<(), Box<dyn Error>> {

    let mut path_json = PathJson::new()?;
    path_json.read()?;

    println!("{:?}", path_json.list);
    rec_check(dir, &mut path_json)?;
    println!("{:?}", path_json.list);

    Ok(())
}

fn rec_check(
    dir: &Path,
    path_json: &mut PathJson,
) -> Result<(), Box<dyn Error>> {
   
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                println!("fichier : {:?}", path);
                path_json.check_file(&path)?;
            } else if path.is_dir() {
                println!("dossier : {:?}", path);
                rec_check(&path, path_json)?;
            }
        }
    }

    Ok(())
}

fn sha256_hash(
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
