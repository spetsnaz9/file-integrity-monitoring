extern crate inotify;

#[macro_use]
extern crate serde_derive;

use inotify::{Inotify, EventMask, WatchDescriptor};
use std::env;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::error::Error;
use notify_rust::Notification;
use chrono::Local;

mod my_error;
use crate::my_error::MyError;
mod event_dir;
use crate::event_dir::{dir_moved_from, dir_moved_to, dir_delete, dir_create};
mod watcher;
use crate::watcher::watch_directory_recursive;
mod init;
use crate::init::{init, sha256_hash};
mod event_file;
use crate::event_file::{check_rec, check_file, write_log};
mod command;
use command::parser;



fn check_path(
    desired_path: &str,
) -> Result<(), ()> {

    if let Ok(current_path) = env::current_dir() {
        if let Ok(desired_canonical_path) = std::fs::canonicalize(desired_path) {
            if current_path.starts_with(desired_canonical_path) {
                return Err(());
            } else {
                return Ok(());
            }
        }
    }

    Err(())
}

fn pop_up (
    content: String,
) -> Result<(), Box<dyn Error>> {
    
    println!("{}", content);

    Notification::new()
        .summary("File Integrity Monitoring")
        .body(&content)
        .show()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Récupère le path à analyser en argument
    let desired_path = parser();

    // Check si ce path est valide
    match check_path(&desired_path) {
        Err(_) => {
            let error = MyError::new("Bad path!");
            return Err(Box::new(error));
        }
        _ => (),
    }

    // Créer la structure path_json contenant les infos sur les fichiers analysés
    let path = Path::new(&desired_path);
    let mut path_json = init(&path)?;

    // Créer des watchers récursif pour chaque dossier dans le path
    let mut inotify = Inotify::init().expect("Failed to initialize inotify");
    let mut watched_dirs: HashMap<WatchDescriptor, PathBuf> = HashMap::new();
    watch_directory_recursive(&inotify, path, &mut watched_dirs)
        .expect("Failed to watch directories");

    let mut buffer = [0; 4096];
    loop {
        let events = inotify.read_events_blocking(&mut buffer).expect("Error while reading events");

        for event in events {
            // Date au format JJ:MM:AAAA HH:MM:SS
            let current_time = Local::now();
            let formatted_time = current_time.format("%d:%m:%Y %H:%M:%S").to_string();

            // Récupération du nom du fichier / dossier ayant un event
            let name = match event.name {
                Some(name) => name,
                None => continue,
            };

            // Récupère le path complet du fichier / dossier ayant un event
            let mut complete_path = match watched_dirs.get(&event.wd) {
                Some(complete_path) => complete_path,
                None => continue
            }.clone();
            complete_path.push(name);

            // Récupère le path du fichier de log pour ce fichier
            let hash = sha256_hash(&complete_path);
            let mut path_save = Path::new("save/").to_path_buf();
            path_save.push(hash);
            path_save.push("log");

            if event.mask.contains(EventMask::ISDIR) { // Si l'event est pour un dossier
                let flag = EventMask::ISDIR ^ event.mask;
                match flag {
                    EventMask::CREATE => {
                        println!("Dossier créé : {:?}", name);
                        dir_create(&inotify, &complete_path, &mut watched_dirs)?;
                        // check_rec(&complete_path, &mut path_json)?;
                    }
                    EventMask::DELETE => {
                        println!("Dossier supprimé : {:?}", name);
                        dir_delete(&complete_path, &mut watched_dirs)?;
                    }
                    EventMask::MOVED_FROM => {
                        println!("Dossier from : {:?}", name);
                        dir_moved_from(&inotify, &complete_path, &mut watched_dirs)?;
                    }
                    EventMask::MOVED_TO => {
                        println!("Dossier to : {:?}", name);
                        dir_moved_to(&inotify, &complete_path, &mut watched_dirs)?;
                        check_rec(&complete_path, &mut path_json)?;
                    }
                    _ => {}
                }
            } else { // Si l'event est pour un fichier
                match event.mask {
                    EventMask::MODIFY => {
                        check_file(&mut path_json, &complete_path, &formatted_time)?;
                        write_log(&path_save, format!("{} : Modified file.\n", formatted_time))?;
                        pop_up(format!("Modified file :\n{:?}", complete_path))?;
                    }
                    EventMask::DELETE => {
                        write_log(&path_save, format!("{} : Deleted file.\n", formatted_time))?;
                        pop_up(format!("Deleted file :\n{:?}", complete_path))?;
                    }
                    EventMask::CREATE => {
                        check_file(&mut path_json, &complete_path, &formatted_time)?;
                        write_log(&path_save, format!("{} : Created file.\n", formatted_time))?;
                        pop_up(format!("Created file :\n{:?}", complete_path))?;
                    }
                    EventMask::MOVED_FROM => {
                        write_log(&path_save, format!("{} : Moved_from file.\n", formatted_time))?;
                        pop_up(format!("Moved_from file :\n{:?}", complete_path))?;
                    }
                    EventMask::MOVED_TO => {
                        check_file(&mut path_json, &complete_path, &formatted_time)?;
                        write_log(&path_save, format!("{} : Moved_to file.\n", formatted_time))?;
                        pop_up(format!("Moved_to file :\n{:?}", complete_path))?;
                    }
                    _ => {}
                }
            }
        }
    }
}
