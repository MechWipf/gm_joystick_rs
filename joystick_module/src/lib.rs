use std::mem::forget;

use sdl2::JoystickSubsystem;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn start() -> Box<Evd>;
        unsafe fn stop(evd: *mut Evd);

        type Evd;
        fn get_num_joysticks(self: &Evd) -> f32;
        fn get_joystick_name(self: &Evd, id: f32) -> String;
        fn get_joystick_guid(self: &Evd, id: f32) -> String;
        fn get_axis(&self, id: f32, id_axis: u32) -> f32;
        fn get_button(&self, id: f32, id_btn: u32) -> bool;
        fn get_pov(&self, id: f32, id_pov: u32) -> f32;
        fn get_num_axes(&self, id: f32) -> f32;
        fn get_num_buttons(&self, id: f32) -> f32;
        fn get_num_povs(&self, id: f32) -> f32;
    }
}

struct Evd {
    joystick: JoystickSubsystem,
}

fn start() -> Box<Evd> {
    let sdl2_context = sdl2::init().unwrap();

    let joystick_module = sdl2_context.joystick().unwrap();
    let evd = Evd {
        joystick: joystick_module,
    };

    forget(sdl2_context);
    Box::new(evd)
}

fn stop(evd: *mut Evd) {
    if evd.is_null() {
        return;
    }

    unsafe {
        _ = Box::from_raw(evd);
    }
}

impl Evd {
    fn get_num_joysticks(&self) -> f32 {
        if let Ok(num_joysticks) = self.joystick.num_joysticks() {
            num_joysticks as f32
        } else {
            0f32
        }
    }

    fn get_joystick_name(&self, id: f32) -> String {
        if let Ok(name) = self.joystick.name_for_index(id as _) {
            name
        } else {
            "".to_owned()
        }
    }

    fn get_joystick_guid(&self, id: f32) -> String {
        if let Ok(name) = self.joystick.device_guid(id as _) {
            name.string()
        } else {
            "".to_owned()
        }
    }

    fn get_axis(&self, id: f32, id_axis: u32) -> f32 {
        if let Ok(device) = self.joystick.open(id as _) {
            if let Ok(axis) = device.axis(id_axis) {
                axis as f32
            } else {
                0f32
            }
        } else {
            0f32
        }
    }

    fn get_button(&self, id: f32, id_btn: u32) -> bool {
        if let Ok(device) = self.joystick.open(id as _) {
            if let Ok(btn) = device.button(id_btn) {
                btn
            } else {
                false
            }
        } else {
            false
        }
    }

    fn get_pov(&self, id: f32, id_pov: u32) -> f32 {
        if let Ok(device) = self.joystick.open(id as _) {
            if let Ok(pov) = device.hat(id_pov) {
                pov as i32 as f32
            } else {
                0f32
            }
        } else {
            0f32
        }
    }

    fn get_num_axes(&self, id: f32) -> f32 {
        if let Ok(device) = self.joystick.open(id as _) {
            device.num_axes() as f32
        } else {
            0f32
        }
    }

    fn get_num_buttons(&self, id: f32) -> f32 {
        if let Ok(device) = self.joystick.open(id as _) {
            device.num_buttons() as f32
        } else {
            0f32
        }
    }

    fn get_num_povs(&self, id: f32) -> f32 {
        if let Ok(device) = self.joystick.open(id as _) {
            device.num_hats() as f32
        } else {
            0f32
        }
    }
}
