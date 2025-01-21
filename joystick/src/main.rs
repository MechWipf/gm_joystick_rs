use std::{
    cell::RefCell,
    collections::HashMap,
    fs::OpenOptions,
    os::unix::fs::OpenOptionsExt,
    path::Path,
    rc::{Rc, Weak},
    thread,
    time::Duration,
};

use evdev_rs::{
    enums::{int_to_ev_abs, int_to_ev_key, EventCode, EventType},
    DeviceWrapper, InputEvent, ReadFlag, UninitDevice,
};
use glob::glob;
use libc::KEY_MAX;

const MAX_DEVICES: i32 = 64;
const MAX_AXES: i32 = 8;
const MAX_BUTTONS: i32 = 32;
const AXIS_MAXIMUM: u32 = 65535;

#[derive(Debug, Clone)]
struct JoyDevice {
    pub name: String,
    pub axes: Vec<Weak<RefCell<u32>>>,
    pub axes_map: HashMap<EventCode, Rc<RefCell<u32>>>,
    pub buttons: Vec<Weak<RefCell<bool>>>,
    pub buttons_map: HashMap<EventCode, Rc<RefCell<bool>>>,
}

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

fn probe_device(path: &Path) -> JoyDevice {
    let mut axes = Vec::new();
    let mut axes_map = HashMap::new();
    let mut buttons = Vec::new();
    let mut buttons_map = HashMap::new();

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(libc::O_NONBLOCK)
        .open(path)
        .expect("Device can be opened.");

    let u_d = UninitDevice::new().expect("Failed to create new device");
    let d = u_d.set_file(file).expect("Failed to create new device");

    println!(
        "Input device ID: bus 0x{:x} vendor 0x{:x} product 0x{:x}",
        d.bustype(),
        d.vendor_id(),
        d.product_id()
    );

    // let mut button_map = HashMap::new();
    if d.has(EventType::EV_KEY) {
        for j in 0..KEY_MAX {
            if let Some(key) = int_to_ev_key(j.clone().into()) {
                let code = EventCode::EV_KEY(key);
                if d.has(code) {
                    let value = Rc::new(RefCell::new(false));
                    buttons.push(Rc::downgrade(&value));
                    buttons_map.insert(code, value);
                }
            }
        }
    } else {
        println!("Device does not have type EV_KEY")
    }

    if d.has(EventType::EV_ABS) {
        for j in 0..KEY_MAX {
            if let Some(key) = int_to_ev_abs(j.clone().into()) {
                let code = EventCode::EV_ABS(key);
                if d.has(code) {
                    let value = Rc::new(RefCell::new(AXIS_MAXIMUM / 2));
                    axes.push(Rc::downgrade(&value));
                    axes_map.insert(code, value);
                }
            }
        }
    } else {
        println!("Device does not have type EV_ABS");
    }

    let joy = JoyDevice {
        name: d.name().unwrap().into(),
        axes,
        axes_map,
        buttons,
        buttons_map,
    };

    println!("{joy:?}");

    joy
}

fn read_events(path: &Path, device: &mut JoyDevice) {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(libc::O_NONBLOCK)
        .open(path)
        .expect("Device can be opened.");

    let u_d = UninitDevice::new().expect("Failed to create new device");
    let d = u_d.set_file(file).expect("Failed to create new device");

    let mut cycle = 0i64;
    loop {
        let a = d.next_event(ReadFlag::NORMAL);
        if let Ok(result) = a {
            match result.0 {
                evdev_rs::ReadStatus::Sync => (),
                evdev_rs::ReadStatus::Success => handle_event(device, &result.1),
            }
        } else {
            cycle = cycle + 1;
            println!(
                "Cycle: {cycle}, {:?}, {:?}",
                device.axes[0].upgrade(),
                device.axes[1].upgrade(),
            );
            thread::sleep(Duration::from_millis(100));
        }
    }
}

fn handle_event(device: &mut JoyDevice, input: &InputEvent) {
    if let Some(btn) = device.buttons_map.get(&input.event_code) {
        btn.replace_with(|_| input.value >= 1);
    }

    if let Some(axe) = device.axes_map.get(&input.event_code) {
        axe.replace_with(|_| {
            let val = input.value + (AXIS_MAXIMUM as i32 / 2);
            let val = val.max(0).min(AXIS_MAXIMUM as i32);
            u32::try_from(val).unwrap()
        });
    }
}
