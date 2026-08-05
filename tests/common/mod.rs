use keyboard_identifier::*;
use std::path::PathBuf;

#[derive(Clone)]
pub struct FakeKeyboard;

impl InputDevice for FakeKeyboard {
    fn is_keyboard(&self) -> bool {
        true
    }
}

pub struct MockSource {
    devices: std::cell::RefCell<Vec<(PathBuf, FakeKeyboard)>>,
}
impl MockSource {
    pub fn new() -> Self {
        Self {
            devices: std::cell::RefCell::new(Vec::new()),
        }
    }

    pub fn plug_keyboard(&self) {
        let index = self.devices.borrow().len();
        let path = PathBuf::from(format!("/dev/input/mock{}", index));
        self.devices.borrow_mut().push((path, FakeKeyboard));
    }

    pub fn unplug_keyboard(&self) {
        self.devices.borrow_mut().pop();
    }
}

impl DeviceSource for MockSource {
    type Device = FakeKeyboard;

    fn enumerate(&self) -> Vec<(PathBuf, FakeKeyboard)> {
        self.devices.borrow().clone()
    }
}
