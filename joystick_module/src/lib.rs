use std::{ mem::forget, thread};

use glob::glob;
use gm_joystick_rs::{probe_device, read_events};

#[cxx::bridge]
mod ffi {
extern "Rust" {
        type Evd;
        fn start() -> Box<Evd>;
    }
}

struct Evd {
}

fn start() -> Box<Evd> {
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

    forget(threads);

    Box::new(Evd {})
}