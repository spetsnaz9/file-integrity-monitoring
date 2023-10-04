extern crate inotify;

use inotify::{Inotify, WatchMask, EventMask, WatchDescriptor};
use std::env;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::error::Error;

mod my_error;
use crate::my_error::MyError;



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

fn add_dir_watch(
    inotify: &Inotify,
    dir: &Path,
    watched_dirs: &mut HashMap<WatchDescriptor, PathBuf>,
) -> Result<(), Box<dyn Error>> {

    let wd = inotify
        .watches()
        .add(
            dir,
            WatchMask::MODIFY | WatchMask::DELETE | WatchMask::CREATE | WatchMask::MOVED_FROM | WatchMask::MOVED_TO,
        )?;

    watched_dirs.insert(wd, dir.to_path_buf());

    Ok(())
}

fn watch_directory_recursive(
    inotify: &Inotify,
    dir: &Path,
    watched_dirs: &mut HashMap<WatchDescriptor, PathBuf>,
) -> Result<(), Box<dyn Error>> {

    let dir_metadata = fs::metadata(dir)?;

    if dir_metadata.is_dir() {
        add_dir_watch(inotify, dir, watched_dirs)?;

        let dir_entries = fs::read_dir(dir)?;
        for entry in dir_entries {
            if let Ok(entry) = entry {
                watch_directory_recursive(inotify, &entry.path(), watched_dirs)?;
            }
        }
    }

    Ok(())
}

fn dir_moved_from (
    inotify: &Inotify,
    complete_path: &PathBuf,
    watched_dirs: &mut HashMap<WatchDescriptor, PathBuf>,
) -> Result<(), Box<dyn Error>> {

    let mut keys_to_remove: Vec<WatchDescriptor> = Vec::new();

    for (key, value) in watched_dirs.iter() {
        if value.starts_with(complete_path) {
            inotify.watches().remove(key.clone())?;
            keys_to_remove.push(key.clone());
        }
    }

    for key in keys_to_remove.iter() {
        watched_dirs.remove(key);
    }

    Ok(())
}

fn dir_delete (
    complete_path: &PathBuf,
    watched_dirs: &mut HashMap<WatchDescriptor, PathBuf>,
) -> Result<(), Box<dyn Error>> {

    let to_remove: WatchDescriptor;

    for (key, value) in watched_dirs.iter() {
        if value.to_path_buf() == complete_path.clone() {
            to_remove = key.clone();
            watched_dirs.remove(&to_remove);
            break;
        }
    }

    Ok(())
}

fn dir_moved_to(
    inotify: &Inotify,
    path: &Path,
    dir: &String,
    watched_dirs: &mut HashMap<WatchDescriptor, PathBuf>,
) -> Result<(), Box<dyn Error>> {

    let mut complete_path = path.to_path_buf();
    complete_path.push(dir);

    watch_directory_recursive(&inotify, &complete_path, watched_dirs)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let desired_path = "/home/spetsnaz/projets/fms/test";
    match check_path(&desired_path) {
        Err(_) => {
            let error = MyError::new("Bad path!");
            return Err(Box::new(error));
        }
        _ => (),
    }

    let path = Path::new(&desired_path);
    let mut inotify = Inotify::init().expect("Failed to initialize inotify");
    let mut watched_dirs: HashMap<WatchDescriptor, PathBuf> = HashMap::new();

    watch_directory_recursive(&inotify, path, &mut watched_dirs)
        .expect("Failed to watch directories");

    let mut buffer = [0; 4096];
    loop {
        let events = inotify.read_events_blocking(&mut buffer).expect("Error while reading events");

        for event in events {
            let name = match event.name {
                Some(name) => name,
                None => continue,
            };

            let mut complete_path = path.to_path_buf();
            complete_path.push(name);

            if event.mask.contains(EventMask::ISDIR) {
                let flag = EventMask::ISDIR ^ event.mask;
                match flag {
                    EventMask::CREATE => {
                        println!("Dossier créé : {:?}", name);
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
                        dir_moved_to(&inotify, path, &name.to_string_lossy().to_string(), &mut watched_dirs)?;
                    }
                    _ => {}
                }
            } else {
                match event.mask {
                    EventMask::MODIFY => {
                        println!("Fichier modifié : {:?}", name);
                    }
                    EventMask::DELETE => {
                        println!("Fichier supprimé : {:?}", name);
                    }
                    EventMask::CREATE => {
                        println!("Fichier créé : {:?}", name);
                    }
                    _ => {}
                }
            }
        }
    }
}
