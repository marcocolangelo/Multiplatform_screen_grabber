use std::collections::HashMap;

use druid::widget::{Button, Controller, Flex, Label, RadioGroup, TextBox};
use druid::Event::KeyDown;
use druid::{Color, Env, Event, EventCtx, Size, Widget, WidgetExt, WindowDesc};
use druid_shell::keyboard_types::Key;
use scrap::Display;

use crate::drawing_area::{self, AppData, MyHotkey};
use crate::function;
use crate::window_format::{self};

struct MyController;
struct MyViewHandler;
impl<W: Widget<AppData>> Controller<AppData, W> for MyViewHandler {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut AppData,
        env: &druid::Env,
    ) {
        match event {
            druid::Event::WindowCloseRequested => {
                if !data.switch_window {
                    ctx.submit_command(druid::commands::QUIT_APP);
                    ctx.set_handled();
                } else {
                    data.switch_window = false;
                }
            }
            _ => {}
        }
        child.event(ctx, event, data, env);
    }
}
impl Controller<String, TextBox<String>> for MyController {
    fn event(
        &mut self,
        child: &mut TextBox<String>,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut String,
        env: &Env,
    ) {
        // serve per far si che il testo inserito sia sempre in minuscolo
        match event {
            KeyDown(_key_event) => {
                if data.len() >= 1 {
                    // facciamo truncate perchè il testo inserito si ripete
                    data.truncate(0);
                }
            }

            _ => {}
        }
        child.event(ctx, event, data, env);
        data.make_ascii_lowercase();
        if ctx.is_disabled() {
            data.clear();
        }
    }
}

pub(crate) fn ui_builder() -> impl Widget<drawing_area::AppData> {
    let save_image = Flex::row()
        .with_child(Label::new("Save your image modifier: "))
        .with_child(
            RadioGroup::row(vec![
                ("Ctrl", "Ctrl".to_string()),
                ("Shift", "Shift".to_string()),
                ("Escape", "Escape".to_string()),
                ("Enter", "Enter".to_string()),
                ("None", "None".to_string()),
            ])
            .border(Color::GRAY, 0.5)
            .lens(drawing_area::AppData::save_image_modifier),
        )
        .with_spacer(30.)
        .with_child(Label::new(" Key: "))
        .with_child(
            TextBox::new()
                .controller(MyController)
                .lens(drawing_area::AppData::save_image_key)
                // disabilita se il modificatore è Escape o Enter
                .disabled_if(|data, _| {
                    data.save_image_modifier == "Escape" || data.save_image_modifier == "Enter"
                })
                .fix_size(26., 26.),
        );
    let entire_screen = Flex::row()
        .with_child(Label::new("Entire screen modifier: "))
        .with_child(
            RadioGroup::row(vec![
                ("Ctrl", "Ctrl".to_string()),
                ("Shift", "Shift".to_string()),
                ("Escape", "Escape".to_string()),
                ("Enter", "Enter".to_string()),
                ("None", "None".to_string()),
            ])
            .border(Color::GRAY, 0.5)
            .lens(drawing_area::AppData::entire_screen_modifier),
        )
        .with_spacer(30.)
        .with_child(Label::new(" Key: "))
        .with_child(
            TextBox::new()
                .controller(MyController)
                .lens(drawing_area::AppData::entire_screen_key)
                .disabled_if(|data, _| {
                    data.entire_screen_modifier == "Escape"
                        || data.entire_screen_modifier == "Enter"
                })
                .fix_size(26., 26.),
        );

    let quit_app = Flex::row()
        .with_child(Label::new("Quit modifier: "))
        .with_child(
            RadioGroup::row(vec![
                ("Ctrl", "Ctrl".to_string()),
                ("Shift", "Shift".to_string()),
                ("Escape", "Escape".to_string()),
                ("Enter", "Enter".to_string()),
                ("None", "None".to_string()),
            ])
            .border(Color::GRAY, 0.5)
            .lens(drawing_area::AppData::quit_app_modifier),
        )
        .with_spacer(30.)
        .with_child(Label::new(" Key: "))
        .with_child(
            TextBox::new()
                .controller(MyController)
                .lens(drawing_area::AppData::quit_app_key)
                .disabled_if(|data, _| {
                    data.quit_app_modifier == "Escape" || data.quit_app_modifier == "Enter"
                })
                .fix_size(26., 26.),
        );

    let edit_image = Flex::row()
        .with_child(Label::new("Edit modifier: "))
        .with_child(
            RadioGroup::row(vec![
                ("Ctrl", "Ctrl".to_string()),
                ("Shift", "Shift".to_string()),
                ("Escape", "Escape".to_string()),
                ("Enter", "Enter".to_string()),
                ("None", "None".to_string()),
            ])
            .border(Color::GRAY, 0.5)
            .lens(drawing_area::AppData::edit_image_modifier),
        )
        .with_spacer(30.)
        .with_child(Label::new(" Key: "))
        .with_child(
            TextBox::new()
                .controller(MyController)
                .lens(drawing_area::AppData::edit_image_key)
                .disabled_if(|data, _| {
                    data.edit_image_modifier == "Escape" || data.edit_image_modifier == "Enter"
                })
                .fix_size(26., 26.),
        );
    let cancel_image = Flex::row()
        .with_child(Label::new("Start drawing modifier: "))
        .with_child(
            RadioGroup::row(vec![
                ("Ctrl", "Ctrl".to_string()),
                ("Shift", "Shift".to_string()),
                ("Escape", "Escape".to_string()),
                ("Enter", "Enter".to_string()),
                ("None", "None".to_string()),
            ])
            .border(Color::GRAY, 0.5)
            .lens(drawing_area::AppData::start_image_modifier),
        )
        .with_spacer(30.)
        .with_child(Label::new(" Key: "))
        .with_child(
            TextBox::new()
                .controller(MyController)
                .lens(drawing_area::AppData::start_image_key)
                .disabled_if(|data, _| {
                    data.start_image_modifier == "Escape" || data.start_image_modifier == "Enter"
                })
                .fix_size(26., 26.),
        );

    let restart = Flex::row()
        .with_child(Label::new("Choose your shortkeys modifier: "))
        .with_child(
            RadioGroup::row(vec![
                ("Ctrl", "Ctrl".to_string()),
                ("Shift", "Shift".to_string()),
                ("Escape", "Escape".to_string()),
                ("Enter", "Enter".to_string()),
                ("None", "None".to_string()),
            ])
            .border(Color::GRAY, 0.5)
            .lens(drawing_area::AppData::restart_app_modifier),
        )
        .with_spacer(30.)
        .with_child(Label::new(" Key: "))
        .with_child(
            TextBox::new()
                .controller(MyController)
                .lens(drawing_area::AppData::restart_app_key)
                .disabled_if(|data, _| {
                    data.restart_app_modifier == "Escape" || data.restart_app_modifier == "Enter"
                })
                .fix_size(26., 26.),
        );
    let choose_format: Flex<drawing_area::AppData> = Flex::row()
        .with_child(Label::new("Choose your format modifier: "))
        .with_child(
            RadioGroup::row(vec![
                ("Ctrl", "Ctrl".to_string()),
                ("Shift", "Shift".to_string()),
                ("Escape", "Escape".to_string()),
                ("Enter", "Enter".to_string()),
                ("None", "None".to_string()),
            ])
            .border(Color::GRAY, 0.5)
            .lens(drawing_area::AppData::restart_format_app_modifier),
        )
        .with_spacer(30.)
        .with_child(Label::new(" Key: "))
        .with_child(
            TextBox::new()
                .controller(MyController)
                .lens(drawing_area::AppData::restart_format_app_key)
                .disabled_if(|data, _| {
                    data.restart_format_app_modifier == "Escape"
                        || data.restart_format_app_modifier == "Enter"
                })
                .fix_size(26., 26.),
        );

    let copy_to_clipboard = Flex::row()
        .with_child(Label::new("Copy modifier: "))
        .with_child(
            RadioGroup::row(vec![
                ("Ctrl", "Ctrl".to_string()),
                ("Shift", "Shift".to_string()),
                ("Escape", "Escape".to_string()),
                ("Enter", "Enter".to_string()),
                ("None", "None".to_string()),
            ])
            .border(Color::GRAY, 0.5)
            .lens(drawing_area::AppData::copy_clipboard_modifier),
        )
        .with_spacer(30.)
        .with_child(Label::new("  Key: "))
        .with_child(
            TextBox::new()
                .controller(MyController)
                .lens(drawing_area::AppData::copy_clipboard_key)
                .disabled_if(|data, _| {
                    data.copy_clipboard_modifier == "Escape"
                        || data.copy_clipboard_modifier == "Enter"
                })
                .fix_size(26., 26.),
        );

    // questa serve a  far si che quando si clicca su apply, si chiuda la finestra e si apra quella di format
    // serve anche a salvare le hotkeys inserite dall'utente in un vettore di struct MyHotkey (vedi crate::drawing_area)
    let apply_button =
        Button::new("Apply").on_click(|ctx, data: &mut drawing_area::AppData, _env| {
            // Qui puoi definire le tue HotKey basate sui valori in data
            data.hotkeys.clear();
            if data.save_image_modifier.eq("Shift") {
                data.save_image_key.make_ascii_uppercase();
            }
            let save_image_modifier = match data.save_image_modifier.as_str() {
                "Ctrl" => Some(Key::Control),
                "Shift" => Some(Key::Shift),
                "Escape" => Some(Key::Escape),
                "Enter" => Some(Key::Enter),
                "None" => None,
                _ => None,
            };

            //key è la combinazione di tasti inserita dall'utente tramite interfaccia grafica
            let key = data.save_image_key.clone();
            let mut shortcut = MyHotkey {
                keys: HashMap::new(),
            };

            // riempiamo shortcut con l'effettiva combinazione di tasti
            // se key.is_empty() è false allora inseriamo la combinazione di tasti
            if !key.is_empty() {
                shortcut
                    .keys
                    .insert(Key::Character(key.clone()), Key::Character(key.clone()));
            }

            // se save_image_modifier è None, allora non inseriamo niente perchè non è stato inserito niente dall'utente
            // verifichiamo sia se è vuota sia che non sia None insomma
            if save_image_modifier != None {
                shortcut.keys.insert(
                    save_image_modifier.clone().unwrap(),
                    save_image_modifier.clone().unwrap(),
                );
            }

            // inseriamo shortcut nel vettore di struct MyHotkey di data
            data.hotkeys.push(shortcut);

            if data.start_image_modifier.eq("Shift") {
                data.start_image_key.make_ascii_uppercase();
            }
            if data.start_image_modifier.eq("Shift") {
                data.start_image_key.make_ascii_uppercase();
            }
            let start_image_modifier = match data.start_image_modifier.as_str() {
                "Ctrl" => Some(Key::Control),
                "Shift" => Some(Key::Shift),
                "Escape" => Some(Key::Escape),
                "Enter" => Some(Key::Enter),
                "None" => None,
                _ => None,
            };
            let mut shortcut = MyHotkey {
                keys: HashMap::new(),
            };
            let key = data.start_image_key.clone();
            if !key.is_empty() {
                shortcut
                    .keys
                    .insert(Key::Character(key.clone()), Key::Character(key.clone()));
            }
            if start_image_modifier != None {
                shortcut.keys.insert(
                    start_image_modifier.clone().unwrap(),
                    start_image_modifier.clone().unwrap(),
                );
            }
            data.hotkeys.push(shortcut);
            if data.quit_app_modifier.eq("Shift") {
                data.quit_app_key.make_ascii_uppercase();
            }
            let quit_app_modifier = match data.quit_app_modifier.as_str() {
                "Ctrl" => Some(Key::Control),
                "Shift" => Some(Key::Shift),
                "Escape" => Some(Key::Escape),
                "Enter" => Some(Key::Enter),
                "None" => None,
                _ => None,
            };
            let mut shortcut = MyHotkey {
                keys: HashMap::new(),
            };
            let key = data.quit_app_key.clone();
            if !key.is_empty() {
                shortcut
                    .keys
                    .insert(Key::Character(key.clone()), Key::Character(key.clone()));
            }
            if quit_app_modifier != None {
                shortcut.keys.insert(
                    quit_app_modifier.clone().unwrap(),
                    quit_app_modifier.clone().unwrap(),
                );
            }
            data.hotkeys.push(shortcut);
            if data.edit_image_modifier.eq("Shift") {
                data.edit_image_key.make_ascii_uppercase();
            }
            let edit_image_modifier = match data.edit_image_modifier.as_str() {
                "Ctrl" => Some(Key::Control),
                "Shift" => Some(Key::Shift),
                "Escape" => Some(Key::Escape),
                "Enter" => Some(Key::Enter),
                "None" => None,
                _ => None,
            };
            let mut shortcut = MyHotkey {
                keys: HashMap::new(),
            };
            let key = data.edit_image_key.clone();
            if !key.is_empty() {
                shortcut
                    .keys
                    .insert(Key::Character(key.clone()), Key::Character(key.clone()));
            }
            if edit_image_modifier != None {
                shortcut.keys.insert(
                    edit_image_modifier.clone().unwrap(),
                    edit_image_modifier.clone().unwrap(),
                );
            }
            data.hotkeys.push(shortcut);

            if data.restart_app_modifier.eq("Shift") {
                data.restart_app_key.make_ascii_uppercase();
            }
            let restart_app_modifier = match data.restart_app_modifier.as_str() {
                "Ctrl" => Some(Key::Control),
                "Shift" => Some(Key::Shift),
                "Escape" => Some(Key::Escape),
                "Enter" => Some(Key::Enter),
                "None" => None,
                _ => None,
            };
            let mut shortcut = MyHotkey {
                keys: HashMap::new(),
            };
            let key = data.restart_app_key.clone();
            if !key.is_empty() {
                shortcut
                    .keys
                    .insert(Key::Character(key.clone()), Key::Character(key.clone()));
            }
            if restart_app_modifier != None {
                shortcut.keys.insert(
                    restart_app_modifier.clone().unwrap(),
                    restart_app_modifier.clone().unwrap(),
                );
            }
            data.hotkeys.push(shortcut);
            if data.restart_format_app_modifier.eq("Shift") {
                data.restart_format_app_key.make_ascii_uppercase();
            }
            let restart_format_app_modifier = match data.restart_format_app_modifier.as_str() {
                "Ctrl" => Some(Key::Control),
                "Shift" => Some(Key::Shift),
                "Escape" => Some(Key::Escape),
                "Enter" => Some(Key::Enter),
                "None" => None,
                _ => None,
            };
            let mut shortcut = MyHotkey {
                keys: HashMap::new(),
            };
            let key = data.restart_format_app_key.clone();
            if !key.is_empty() {
                shortcut
                    .keys
                    .insert(Key::Character(key.clone()), Key::Character(key.clone()));
            }
            if restart_format_app_modifier != None {
                shortcut.keys.insert(
                    restart_format_app_modifier.clone().unwrap(),
                    restart_format_app_modifier.clone().unwrap(),
                );
            }
            data.hotkeys.push(shortcut);

            if data.copy_clipboard_modifier.eq("Shift") {
                data.copy_clipboard_key.make_ascii_uppercase();
            }
            let copy_clipboard_modifier = match data.copy_clipboard_modifier.as_str() {
                "Ctrl" => Some(Key::Control),
                "Shift" => Some(Key::Shift),
                "Escape" => Some(Key::Escape),
                "Enter" => Some(Key::Enter),
                "None" => None,
                _ => None,
            };
            let mut shortcut = MyHotkey {
                keys: HashMap::new(),
            };
            let key = data.copy_clipboard_key.clone();
            if !key.is_empty() {
                shortcut
                    .keys
                    .insert(Key::Character(key.clone()), Key::Character(key.clone()));
            }
            if copy_clipboard_modifier != None {
                shortcut.keys.insert(
                    copy_clipboard_modifier.clone().unwrap(),
                    copy_clipboard_modifier.clone().unwrap(),
                );
            }
            data.hotkeys.push(shortcut);

            if data.entire_screen_modifier.eq("Shift") {
                data.entire_screen_key.make_ascii_uppercase();
            }
            let entire_screen_modifier = match data.entire_screen_modifier.as_str() {
                "Ctrl" => Some(Key::Control),
                "Shift" => Some(Key::Shift),
                "Escape" => Some(Key::Escape),
                "Enter" => Some(Key::Enter),
                "None" => None,
                _ => None,
            };
            let key = data.entire_screen_key.clone();
            let mut shortcut = MyHotkey {
                keys: HashMap::new(),
            };
            if !key.is_empty() {
                shortcut
                    .keys
                    .insert(Key::Character(key.clone()), Key::Character(key.clone()));
            }
            if entire_screen_modifier != None {
                shortcut.keys.insert(
                    entire_screen_modifier.clone().unwrap(),
                    entire_screen_modifier.clone().unwrap(),
                );
            }
            data.hotkeys.push(shortcut);

            // qui si apre la finestra di format se tutti i campi sono compilati e se non ci sono shortkeys uguali
            let format_window = WindowDesc::new(window_format::build_ui())
                .transparent(false)
                .title("Choose the format. Default is .png")
                .window_size(Size::new(400.0, 400.0))
                .set_position(druid::Point::new(500., 300.))
                .set_always_on_top(true);

            // se tutti i campi sono compilati e se non ci sono shortkeys uguali, allora si apre la finestra di format
            if function::are_all_fields_completed(data) && !function::some_fields_are_equal(data) {
                if data.show_drawing {
                    let display_primary = Display::primary().expect("error");
                    let main_window = WindowDesc::new(drawing_area::build_ui())
                        .with_min_size(Size::new(
                            display_primary.width() as f64,
                            display_primary.height() as f64,
                        ))
                        .show_titlebar(false)
                        .set_position(druid::Point::new(0., 0.))
                        .window_size(Size::new(
                            display_primary.width() as f64,
                            display_primary.height() as f64,
                        ))
                        .resizable(true)
                        //.show_titlebar(false)
                        .set_always_on_top(true)
                        .transparent(true)
                        .set_window_state(druid_shell::WindowState::Maximized);

                    // let id = main_window.id.clone();
                    ctx.new_window(main_window);
                } else {
                    ctx.new_window(format_window);
                }

                // switch_window serve per far si che quando si clicca su apply, si chiuda la finestra e si apra quella di format
                data.switch_window = true;
                ctx.submit_command(druid::commands::CLOSE_WINDOW.to(ctx.window_id()));

                ctx.set_handled();
                // ctx.submit_command(druid::commands::SHOW_WINDOW.to(data.format_window_id));
            }
        });
    let errore = Label::new(|data: &drawing_area::AppData, _env: &Env| {
        if function::are_all_fields_completed(data) {
            "".to_string()
        } else {
            "Per favore, compila tutti i campi.".to_string()
        }
    });

    let errore_field = Label::new(|data: &drawing_area::AppData, _env: &Env| {
        if function::some_fields_are_equal(data) {
            "Stesse shortkeys non sono ammesse".to_string()
        } else {
            "".to_string()
        }
    });

    Flex::column()
        .with_child(Label::new("Choose your shortkeys: "))
        .with_child(errore)
        .with_child(errore_field)
        .with_child(save_image)
        .with_spacer(20.)
        .with_child(quit_app)
        .with_spacer(20.)
        .with_child(edit_image)
        .with_spacer(20.)
        .with_child(entire_screen)
        .with_spacer(20.)
        .with_child(cancel_image)
        .with_spacer(20.)
        .with_child(restart)
        .with_spacer(20.)
        .with_child(choose_format)
        .with_spacer(20.)
        .with_child(copy_to_clipboard)
        .with_spacer(20.)
        .with_child(apply_button)
        .controller(MyViewHandler)
}
