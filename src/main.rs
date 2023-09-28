extern crate inotify;

use inotify::{Inotify, WatchMask, EventMask};
use std::env;



fn check_path(desired_path: &str) -> Result<(), ()> {
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


fn main() {
    let desired_path = "/home/spetsnaz/projets/fms/test";
    match check_path(&desired_path) {
        Err(_) => {
            println!("Bad path");
            return;
        },
        _ => ()
    }

    let mut inotify = Inotify::init()
        .expect("Failed to initialize inotify");

    inotify
        .watches()
        .add(
            desired_path, 
            WatchMask::MODIFY | WatchMask::DELETE | WatchMask::CREATE)
        .expect("Failed to add notify watch");

    let mut buffer = [0; 4096];
    loop {
        let events = inotify
            .read_events_blocking(&mut buffer)
            .expect("Error while reading events");

        for event in events {
            if !event.mask.contains(EventMask::ISDIR) {
                let name_file = match event.name {
                    Some(name_file) => name_file,
                    None => continue
                };

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
