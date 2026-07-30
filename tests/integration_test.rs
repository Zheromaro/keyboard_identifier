use keyboard_identifier::*;

#[test]
fn test_identifies_separate_keyboards() {
    let mut tracker = KeyboardTracker::new();

    // 1. Simulate typing "A" (0x41) on Keyboard #1 (Primary Keyboard)
    let kb1_event = KeyboardEvent {
        device_id: "\\\\?\\HID#VID_046D&PID_C52B#1".to_string(),
        key_code: 0x41,
    };

    // 2. Simulate typing "B" (0x42) on Keyboard #2 (Secondary / Numpad)
    let kb2_event = KeyboardEvent {
        device_id: "\\\\?\\HID#VID_1A2B&PID_3C4D#2".to_string(),
        key_code: 0x42,
    };

    tracker.process_event(kb1_event);
    tracker.process_event(kb2_event);

    // Assert that events are segregated correctly by device handle
    assert_eq!(
        tracker.history.get("\\\\?\\HID#VID_046D&PID_C52B#1"),
        Some(&vec![0x41])
    );
    assert_eq!(
        tracker.history.get("\\\\?\\HID#VID_1A2B&PID_3C4D#2"),
        Some(&vec![0x42])
    );
}
