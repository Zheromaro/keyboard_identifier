use evdev::{Device, KeyCode};
use std::path::PathBuf;

pub trait InputDevice {
    fn is_keyboard(&self) -> bool;
}

pub trait DeviceSource {
    type Device: InputDevice;
    fn enumerate(&self) -> Vec<(PathBuf, Self::Device)>;
}

// Linux
pub struct LinuxDeviceSource;

impl InputDevice for Device {
    fn is_keyboard(&self) -> bool {
        self.supported_keys()
            .map(|keys| keys.contains(KeyCode::KEY_A) && keys.contains(KeyCode::KEY_ENTER))
            .unwrap_or(false)
    }
}
impl DeviceSource for LinuxDeviceSource {
    type Device = Device;

    fn enumerate(&self) -> Vec<(PathBuf, Device)> {
        evdev::enumerate().collect()
    }
}

pub fn get_keyboard_list<S: DeviceSource>(source: &S) -> Vec<PathBuf> {
    source
        .enumerate()
        .into_iter()
        .filter(|(_, device)| device.is_keyboard())
        .map(|(path, _)| path)
        .collect()
}
