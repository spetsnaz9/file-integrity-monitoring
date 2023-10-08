extern crate inotify;

#[macro_use]
extern crate serde_derive;

use inotify::{Inotify, EventMask, WatchDescriptor};
use std::env;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::error::Error;

mod my_error;
use crate::my_error::MyError;
mod event_dir;
use crate::event_dir::{dir_moved_from, dir_moved_to, dir_delete, dir_create};
mod watcher;
use crate::watcher::watch_directory_recursive;
mod init;
use crate::init::init;
mod tracker_file;
use tracker_file::{check_rec, check_file};



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
    let mut path_json = init(&path)?;

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

            let mut complete_path = match watched_dirs.get(&event.wd) {
                Some(complete_path) => complete_path,
                None => continue
            }.clone();
            complete_path.push(name);

            if event.mask.contains(EventMask::ISDIR) {
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
                        check_file(&mut path_json, &complete_path)?;
                    }
                    _ => {}
                }
            }
        }
    }
}
