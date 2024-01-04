use std::{
    collections::HashMap,
    sync::{Arc, Condvar, Mutex},
};

use drawing_area::Conventions;
use druid::{LocalizedString, Point};
use image::ImageBuffer;
use window_format::MyRadio;

use druid::{AppLauncher, Rect, WindowDesc};

use druid_shell::keyboard_types::Key;

mod convention_window;
mod drawing_area;
mod function;
mod information_window;
mod screenshot;
mod shortkeys_window;
mod window_format;
fn main() {
    let main_window = WindowDesc::new(shortkeys_window::ui_builder())
        .title(LocalizedString::new("Keyboard Shortcut Settings"))
        .window_size((1000.0, 1000.0));

    let initial_state = drawing_area::AppData {
        save_image_modifier: "Enter".into(),
        save_image_key: (Key::Character("".to_string())).to_string(),
        quit_app_modifier: "Escape".into(),
        quit_app_key: (Key::Character("".to_string())).to_string(),
        edit_image_modifier: "None".into(),
        edit_image_key: (Key::Character("m".to_string())).to_string(),
        start_image_modifier: "None".into(),
        start_image_key: (Key::Character("s".to_string())).to_string(),
        entire_screen_modifier: "None".into(),
        entire_screen_key: (Key::Character("a".to_string())).to_string(),
        restart_app_modifier: "Shift".into(),
        restart_app_key: (Key::Character("".to_string())).to_string(),
        restart_format_app_modifier: "Ctrl".into(),
        restart_format_app_key: (Key::Character("".to_string())).to_string(),
        hotkeys: Vec::new(),
        is_selecting: false,
        start_position: None,
        end_position: None,
        start_position_to_display: None,
        end_position_to_display: None,
        modify: false,
        is_dragging: false,
        rect: Rect::new(0.0, 0.0, 0.0, 0.0),
        where_dragging: None,
        radio_group: MyRadio::Png,
        label: "screenshot_grabbed".to_string(),
        switch_window: false,
        is_found: false,
        last_key_event: None,
        hide_buttons: false,
        counter: 0,
        capture_screen: false,
        tasti: HashMap::new(),
        attivazione: HashMap::new(),
        count: 0,
        myimage: ImageBuffer::new(0, 0),
        show_drawing: false,
        copy_clipboard_modifier: "Ctrl".into(),
        copy_clipboard_key: (Key::Character("s".to_string())).to_string(),
        file_path: "screenshot_grabbed".to_string(),
        my_convention: Conventions::DefaultConvention,
        myselector: Arc::new((Mutex::new(false), Condvar::new())),
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}
