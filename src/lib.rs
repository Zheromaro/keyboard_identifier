type KeyboardID = String;

fn keyboard_listener() {
    // TODO: make an infinite loop
    // TODO: upadte keyboards list (check if a keyboard in unpluged or pluged)
    // TODO: if a key is presed: indentify the keyboard, get the char, and call on_key_pressed(id: KeyboardID, c: char)
}

pub fn init_keyboard_listener() {
    // TODO: start the keyboard listener thread
}

pub fn end_keyboard_listener() {
    // TODO: stop the keyboard listener thread
}

pub fn on_key_pressed(id: KeyboardID, c: char) -> (KeyboardID, char) {
    (id, c)
}
