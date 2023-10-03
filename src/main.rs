extern crate inotify;

use inotify::{Inotify, WatchMask, EventMask};
use std::env;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};



fn check_path(
    desired_path: &str
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

fn watch_directory_recursive(
    inotify: &Inotify,
    dir: &Path,
    watched_dirs: &mut HashMap<i32, PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {

    let dir_metadata = fs::metadata(dir)?;

    if dir_metadata.is_dir() {
        let wd = inotify
            .watches()
            .add(
                dir,
                WatchMask::MODIFY | WatchMask::DELETE | WatchMask::CREATE,
            )?;

        let wd_id: i32 = wd.get_watch_descriptor_id();
        watched_dirs.insert(wd_id, dir.to_path_buf());
        let dir_entries = fs::read_dir(dir)?;

        for entry in dir_entries {
            if let Ok(entry) = entry {
                watch_directory_recursive(inotify, &entry.path(), watched_dirs)?;
            }
        }
    }

    Ok(())
}

fn main() {
    let desired_path = "/home/spetsnaz/projets/fms/test";
    match check_path(&desired_path) {
        Err(_) => {
            println!("Bad path");
            return;
        }
        _ => (),
    }

    let mut inotify = Inotify::init().expect("Failed to initialize inotify");
    let mut watched_dirs: HashMap<i32, PathBuf> = HashMap::new();

    watch_directory_recursive(&inotify, Path::new(&desired_path), &mut watched_dirs)
        .expect("Failed to watch directories");

    let mut buffer = [0; 4096];
    loop {
        let events = inotify.read_events_blocking(&mut buffer).expect("Error while reading events");

        for event in events {
            let name_file = match event.name {
                Some(name_file) => name_file,
                None => continue,
            };

            if event.mask.contains(EventMask::ISDIR) {
                if event.mask.contains(EventMask::CREATE) {
                    println!("Dossier créé : {:?}", name_file);
                } else if event.mask.contains(EventMask::DELETE) {
                    println!("Dossier supprimé : {:?}", name_file);
                }
            } else {
                match event.mask {
                    EventMask::MODIFY => {
                        println!("Fichier modifié : {:?}", name_file);
                    }
                    EventMask::DELETE => {
                        println!("Fichier supprimé : {:?}", name_file);
                    }
                    EventMask::CREATE => {
                        println!("Fichier créé : {:?}", name_file);
                    }
                    _ => {}
                }
            }
        }
    }
}
