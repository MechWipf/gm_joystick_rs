use std::thread;

use glob::glob;
use gm_joystick_rs::{probe_device, read_events};

fn main() {
    let mut threads = vec![];

    let devices = glob("/dev/input/by-id/*-event-joystick").expect("Failed to read glob pattern");
    for entry in devices {
        match entry {
            Ok(path) => {
                println!("{path:?}");

                let t = thread::spawn(move || {
                    let mut device = probe_device(&path);
                    if device.name.contains("Microsoft") {
                        read_events(&path, &mut device);
                    }
                });
                threads.push(t);
            }
            Err(e) => println!("Error: {e:?}"),
        }
    }

    for t in threads {
        t.join().unwrap()
    }
}
