use std::path::Path;
use std::error::Error;
use std::fs;

use crate::init::{PathJson, ContentJson, sha256_hash};


fn check_file(
    path_json: &mut PathJson,
    path: &Path,
    ) -> Result<(), Box<dyn Error>> {

    if !path_json.exist.contains(path) {
        let hash = sha256_hash(path);

        let new = ContentJson {
            path: path.to_string_lossy().to_string(),
            hash,
        };

        path_json.list.push(new);            
        path_json.exist.insert(path.to_path_buf());

        path_json.write()?;
    }

    Ok(())
}

pub fn check_rec(
    dir: &Path,
    path_json: &mut PathJson,
) -> Result<(), Box<dyn Error>> {
   
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                println!("fichier : {:?}", path);
                check_file(path_json, &path)?;
            } else if path.is_dir() {
                println!("dossier : {:?}", path);
                check_rec(&path, path_json)?;
            }
        }
    }

    Ok(())
}

