use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::error::Error;
use std::fs::{self, File};

use crate::init::{PathJson, ContentJson, sha256_hash};



pub fn check_file(
    path_json: &mut PathJson,
    path: &PathBuf,
    ) -> Result<(), Box<dyn Error>> {

    if !path_json.exist.contains(path) {
        let hash = sha256_hash(path);

        let new = ContentJson {
            path: path.to_string_lossy().to_string(),
            hash: hash.clone(),
        };

        path_json.list.push(new);            
        path_json.exist.insert(path.to_path_buf());

        path_json.write()?;

        let mut new_dir_save = Path::new("save/").to_path_buf();
        new_dir_save.push(hash);
        let new_file_save = new_dir_save.join("log");
        
        if !new_dir_save.exists() {
            fs::create_dir(&new_dir_save)?;
        }

        File::create(new_file_save)?;

        new_dir_save.push("copy");
        fs::copy(path, new_dir_save)?;
    }
    // Sinon check les modifications qu'il y a eu entre temps

    Ok(())
}

pub fn check_rec(
    dir: &PathBuf,
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
