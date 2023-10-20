use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::error::Error;
use std::fs::{self, File};
use chrono::Local;
use diffy::create_patch;

use crate::init::{PathJson, ContentJson, sha256_hash};



fn create_file(
    path_json: &mut PathJson,
    path: &PathBuf,
    hash: String,
    new_dir_save: &mut PathBuf,
) -> Result<(), Box<dyn Error>> {

    let path_string = path.to_string_lossy().to_string();

    let new = ContentJson {
        path: path_string.clone(),
        hash: hash.clone(),
    };

    path_json.list.push(new);            
    path_json.exist.insert(path.to_path_buf());
    path_json.write()?;

    let new_file_save = new_dir_save.join("log");

    if !new_dir_save.exists() {
        fs::create_dir(&new_dir_save)?;
    }

    File::create(new_file_save)?;

    new_dir_save.push("copy");
    fs::copy(path, new_dir_save)?;

    Ok(())
}

fn modify_file(
    path: &PathBuf,
    formatted_time: &String,
    new_dir_save: &mut PathBuf,
) -> Result<(), Box<dyn Error>> {

    let new_file_save = new_dir_save.join("copy");
    let new_file_diff = new_dir_save.join(format!("diff_{}.txt", formatted_time));
    let new_file_copy = new_dir_save.join("copy");

    let old_content = fs::read_to_string(new_file_save)?;
    let new_content = fs::read_to_string(path)?;

    let patch = create_patch(&old_content, &new_content);

    let mut file = File::create(&new_file_diff)?;
    file.write_all(&patch.to_bytes())?;

    fs::copy(path, new_file_copy)?;

    Ok(())
}

pub fn check_file(
    path_json: &mut PathJson,
    path: &PathBuf,
    formatted_time: &String,
) -> Result<(), Box<dyn Error>> {

    // Récupère le dossier de save pour ce fichier
    let hash = sha256_hash(path);
    let mut new_dir_save = Path::new("save/").to_path_buf();
    new_dir_save.push(hash.clone());

    if !path_json.exist.contains(path) { // Si le fichier est créé
        create_file(path_json, path, hash, &mut new_dir_save)?;
    } else { // Si le fichier est modifié
        modify_file(path, formatted_time, &mut new_dir_save)?;
    }

    Ok(())
}

pub fn check_rec(
    dir: &PathBuf,
    path_json: &mut PathJson,
) -> Result<(), Box<dyn Error>> {
   
    // Date au format JJ:MM:AAAA HH:MM:SS
    let current_time = Local::now();
    let formatted_time = current_time.format("%d:%m:%Y %H:%M:%S").to_string();

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                check_file(path_json, &path, &formatted_time)?;
            } else if path.is_dir() {
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
