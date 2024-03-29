use std::sync::{Arc, Condvar, Mutex};

use arboard::ImageData;
use std::{collections::HashMap, env};

use druid::widget::{
    BackgroundBrush, Button, Controller, Flex, Image, Label, Padding, ViewSwitcher,
};
use druid::{
    Color, Data, Env, Event, EventCtx, FileDialogOptions, ImageBuf, Insets, Lens, PaintCtx, Point,
    Rect, RenderContext, Selector, Size, Widget, WidgetExt, WindowDesc,
};

use druid_shell::{keyboard_types::Key, piet::ImageFormat, KeyEvent, MouseButton};

use crate::convention_window;
use crate::function;
use crate::information_window;
use crate::screenshot::{self};
use crate::shortkeys_window;
use crate::window_format;
use image::{EncodableLayout, ImageBuffer, Rgba};
use scrap::Display;
use std::thread;
use std::time::Duration;

// Lens serve a creare una struttura dati che può essere modificata in modo sicuro da più thread
//druid::Data invece serve a usare una struttura per il context di druid

#[derive(Clone, Data, Lens)]
pub struct AppData {
    pub(crate) is_selecting: bool,
    pub(crate) start_position: Option<Point>,
    pub(crate) end_position: Option<Point>,
    pub(crate) start_position_to_display: Option<Point>,
    pub(crate) end_position_to_display: Option<Point>,
    pub(crate) modify: bool,
    pub(crate) is_dragging: bool,
    pub(crate) rect: Rect,
    pub(crate) where_dragging: Option<DragHandle>,
    pub(crate) radio_group: window_format::MyRadio,
    pub(crate) label: String,
    pub(crate) save_image_modifier: String,
    pub(crate) save_image_key: String,
    pub(crate) quit_app_modifier: String,
    pub(crate) quit_app_key: String,
    pub(crate) edit_image_modifier: String,
    pub(crate) edit_image_key: String,
    pub(crate) start_image_modifier: String,
    pub(crate) start_image_key: String,
    pub(crate) restart_app_modifier: String,
    pub(crate) restart_app_key: String,
    pub(crate) restart_format_app_modifier: String,
    pub(crate) restart_format_app_key: String,
    pub(crate) entire_screen_modifier: String,
    pub(crate) entire_screen_key: String,
    pub(crate) is_found: bool,
    pub(crate) hide_buttons: bool,
    pub(crate) switch_window: bool,
    pub(crate) show_drawing: bool,
    pub(crate) capture_screen: bool,
    pub(crate) copy_clipboard_modifier: String,
    pub(crate) copy_clipboard_key: String,
    pub(crate) file_path: String,
    pub(crate) counter: i32,
    pub(crate) my_convention: Conventions,
    pub(crate) myselector: Arc<(Mutex<bool>, Condvar)>,
    #[data(ignore)]
    pub(crate) myimage: ImageBuffer<Rgba<u8>, Vec<u8>>,
    #[data(ignore)]
    pub(crate) hotkeys: Vec<MyHotkey>,
    #[data(ignore)]
    pub(crate) last_key_event: Option<KeyEvent>,
    #[data(ignore)]
    pub(crate) tasti: HashMap<Key, Key>,
    #[data(ignore)]
    pub(crate) attivazione: HashMap<Key, Key>,
    pub(crate) count: i32,
}

// Definisci la struttura della tua hotkey

#[derive(Clone, PartialEq, Debug)]
pub struct MyHotkey {
    pub(crate) keys: HashMap<Key, Key>,
}

#[derive(Clone, Data, PartialEq)]
pub enum DragHandle {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}
#[derive(Clone, Data, PartialEq, Copy, Debug)]
pub enum Conventions {
    TimeConvention,
    DefaultConvention,
    NumericConvention,
}

struct DrawingArea;

static ENTIRE_SCREEN: Selector = Selector::new("ENTIRE SCREEN");
impl Widget<AppData> for DrawingArea {
    fn event(&mut self, ctx: &mut EventCtx, event: &druid::Event, data: &mut AppData, _env: &Env) {
        ctx.request_focus();
        match event {
            // comando per catturare l'intero schermo
            druid::Event::Command(cmd) if cmd.is(ENTIRE_SCREEN) => {
                //sleep per il passagio da un widget all'altro
                thread::sleep(Duration::from_millis(500));

                // capture_screen serve a definire se da qualche parte si è detto di cattu4rare lo schermo
                if data.capture_screen {
                    let start_position = Some(Point { x: 0., y: 0. });
                    let end_position = Some(Point::new(
                        Display::primary().expect("err").width() as f64,
                        Display::primary().expect("err").height() as f64,
                    ));

                    // cattura lo schermo e salva l'immagine in data.myimage come variabile temporanea
                    data.myimage =
                        screenshot::screen_new(start_position.unwrap(), end_position.unwrap());
                    data.capture_screen = false;
                    data.hide_buttons = false;
                    data.start_position = None;
                    data.end_position = None;
                }
            }

            // Evento che si attiva quando si clicca con il mouse
            druid::Event::MouseDown(mouse_event) => {
                if data.modify == true && data.is_dragging == false {
                    data.start_position = None;
                    data.end_position = None;
                    data.start_position_to_display = None;
                    data.end_position_to_display = None;
                    data.is_selecting = false;
                    data.is_dragging = false;
                    data.modify = false;
                }

                // se non si sta modificando il rettangolo e non si sta trascinando il rettangolo allora si può selezionare un rettangolo
                if data.modify == false && data.is_dragging == false {
                    if mouse_event.button == MouseButton::Left {
                        data.start_position = None;
                        data.end_position = None;
                        let os = env::consts::OS;
                        match os {
                            "windows" => {
                                // gestisci il fattore di scala per il mouse event perchè è in coordinate dello schermo e non del widget druid
                                let scale_factor_x = ctx.window().get_scale().unwrap().x();
                                let scale_factor_y = ctx.window().get_scale().unwrap().y();
                                let coord = druid::Point {
                                    x: mouse_event.pos.x * scale_factor_x,
                                    y: mouse_event.pos.y * scale_factor_y,
                                };
                                data.start_position_to_display = Some(druid::Point {
                                    x: mouse_event.pos.x,
                                    y: mouse_event.pos.y,
                                });
                                data.start_position = Some(coord);
                            }
                            _ => {
                                let pos = ctx.to_screen(druid::Point::new(
                                    mouse_event.pos.x,
                                    mouse_event.pos.y,
                                ));
                                //let size=ctx.window().get_size();
                                //println!("size: {:?}",size);
                                let coord = druid::Point { x: pos.x, y: pos.y };
                                data.start_position_to_display = Some(druid::Point {
                                    x: mouse_event.pos.x,
                                    y: mouse_event.pos.y,
                                });
                                data.start_position = Some(coord);
                            }
                        }

                        //println!("Click su pos: {:?}",mouse_event.pos);
                        // println!("Click su window_pos: {:?}",mouse_event.window_pos);

                        data.is_selecting = true;
                    }
                }
                if data.is_dragging == true {
                    //println!("{:?}",(mouse_event.pos - data.rect.origin()).hypot());

                    // se si sta trascinando il rettangolo allora si può modificare il rettangolo solo se si clicca su uno dei 4 angoli del rettangolo
                    if (mouse_event.pos - data.rect.origin()).hypot() < 70.0 {
                        ctx.set_cursor(&druid::Cursor::Crosshair);
                        data.where_dragging = Some(DragHandle::TopLeft);
                        ctx.set_active(true);
                    } else if (mouse_event.pos - Point::new(data.rect.x1, data.rect.y1)).hypot()
                        < 70.0
                    {
                        ctx.set_cursor(&druid::Cursor::Crosshair);
                        data.where_dragging = Some(DragHandle::BottomRight);
                        ctx.set_active(true);
                    } else if (mouse_event.pos - Point::new(data.rect.x0, data.rect.y1)).hypot()
                        < 70.0
                    {
                        ctx.set_cursor(&druid::Cursor::Crosshair);
                        data.where_dragging = Some(DragHandle::BottomLeft);
                        ctx.set_active(true);
                    } else if (mouse_event.pos - Point::new(data.rect.x1, data.rect.y0)).hypot()
                        < 70.0
                    {
                        ctx.set_cursor(&druid::Cursor::Crosshair);
                        data.where_dragging = Some(DragHandle::TopRight);
                        ctx.set_active(true);
                    } else {
                        data.hide_buttons = false;
                    }
                    data.is_selecting = true;
                }
            }

            druid::Event::MouseMove(mouse_event) => {
                // Aggiorna la posizione finale del rettangolo qui

                let os = env::consts::OS;
                match os {
                    "windows" => {
                        if ctx.is_active() == false && data.is_dragging == false {
                            let scale_factor_x = ctx.window().get_scale().unwrap().x();
                            let scale_factor_y = ctx.window().get_scale().unwrap().y();
                            let coord = druid::Point {
                                x: mouse_event.pos.x * scale_factor_x,
                                y: mouse_event.pos.y * scale_factor_y,
                            };
                            data.end_position_to_display = Some(druid::Point {
                                x: mouse_event.pos.x,
                                y: mouse_event.pos.y,
                            });
                            data.end_position = Some(coord);
                        }

                        // richiede di aggiornare il widget per ridisegnare il rettangolo con la nuova posizione finale del rettangolo
                        if ctx.is_active() {
                            let scale_factor_x = ctx.window().get_scale().unwrap().x();
                            let scale_factor_y = ctx.window().get_scale().unwrap().y();
                            if let Some(handle) = &data.where_dragging.clone() {
                                // let scale_factor_x = ctx.window().get_scale().unwrap().x();
                                // let scale_factor_y = ctx.window().get_scale().unwrap().y();
                                let coord = druid::Point {
                                    x: mouse_event.pos.x * scale_factor_x,
                                    y: mouse_event.pos.y * scale_factor_y,
                                };

                                function::edit_rect(handle, coord, data, mouse_event);
                                ctx.request_paint();
                            }
                        }
                    }
                    _ => {
                        if ctx.is_active() == false && data.is_dragging == false {
                            let pos = ctx
                                .to_screen(druid::Point::new(mouse_event.pos.x, mouse_event.pos.y));

                            let coord = druid::Point { x: pos.x, y: pos.y };
                            data.end_position_to_display = Some(druid::Point {
                                x: mouse_event.pos.x,
                                y: mouse_event.pos.y,
                            });
                            data.end_position = Some(coord);
                        }

                        if ctx.is_active() {
                            if let Some(handle) = &data.where_dragging.clone() {
                                let pos = ctx.to_screen(druid::Point::new(
                                    mouse_event.pos.x,
                                    mouse_event.pos.y,
                                ));
                                function::edit_rect(handle, pos, data, mouse_event);

                                ctx.request_paint();
                            }
                        }
                    }
                }

                // Richiedi un aggiornamento del widget per ridisegnare il rettangolo

                if data.modify == false {
                    ctx.request_paint();
                }
            }

            // Evento che si attiva quando si rilascia il mouse
            druid::Event::MouseUp(mouse_event) => {
                if data.is_dragging == true {
                    data.where_dragging = None;
                    ctx.set_active(false);
                    data.is_selecting = true;
                }
                if data.modify == false && data.is_dragging == false {
                    if mouse_event.button == MouseButton::Left {
                        data.is_selecting = false;
                        data.modify = true;

                        let os = env::consts::OS;
                        match os {
                            "windows" => {
                                // prima di salvare il rettangolo verifica che è tutto okay e aggiusta dove devi aggiustare
                                let scale_factor_x = ctx.window().get_scale().unwrap().x();
                                let scale_factor_y = ctx.window().get_scale().unwrap().y();
                                let coord = druid::Point {
                                    x: mouse_event.pos.x * scale_factor_x,
                                    y: mouse_event.pos.y * scale_factor_y,
                                };
                                data.end_position_to_display = Some(druid::Point {
                                    x: mouse_event.pos.x,
                                    y: mouse_event.pos.y,
                                });
                                if coord.x < data.start_position.unwrap().x
                                    && coord.y < data.start_position.unwrap().y
                                {
                                    let prov = data.start_position;
                                    data.start_position = Some(coord);
                                    data.end_position = prov;
                                    let prov_display = data.start_position_to_display;
                                    data.start_position_to_display = data.end_position_to_display;
                                    data.end_position_to_display = prov_display;
                                } else {
                                    data.end_position = Some(coord);
                                }

                                // Calcola il rettangolo selezionato e salvalo in data.rect
                                data.rect = druid::Rect::from_points(
                                    data.start_position_to_display.unwrap(),
                                    data.end_position_to_display.unwrap(),
                                );
                            }
                            _ => {
                                let pos = ctx.to_screen(druid::Point::new(
                                    mouse_event.pos.x,
                                    mouse_event.pos.y,
                                ));

                                let coord = druid::Point { x: pos.x, y: pos.y };

                                data.end_position_to_display = Some(druid::Point {
                                    x: mouse_event.pos.x,
                                    y: mouse_event.pos.y,
                                });
                                if coord.x < data.start_position.unwrap().x
                                    && coord.y < data.start_position.unwrap().y
                                {
                                    let prov = data.start_position;
                                    data.start_position = Some(coord);
                                    data.end_position = prov;
                                    let prov_display = data.start_position_to_display;
                                    data.start_position_to_display = data.end_position_to_display;
                                    data.end_position_to_display = prov_display;
                                } else {
                                    data.end_position = Some(coord);
                                }
                                data.rect = druid::Rect::from_points(
                                    data.start_position.unwrap(),
                                    data.end_position.unwrap(),
                                );
                            }
                        }
                    }

                    data.hide_buttons = false;
                }
                // println!("{:?}",data.rect);
                if data.start_position != None
                    && data.end_position != None
                    && data.start_position != data.end_position
                {
                    data.myimage = screenshot::screen_new(
                        data.start_position.unwrap(),
                        data.end_position.unwrap(),
                    );
                }
            }

            _ => {}
        }
    }

    // Questa funzione serve a definire il ciclo di vita del widget e a definire cosa fare quando
    // si aggiorna il widget (quando si ridisegna) e quando si cambia la dimensione del widget
    // ma noi lo usiamo principalemnte per inviare messaggio al thread principale di cattura dello schermo
    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        _event: &druid::LifeCycle,
        data: &AppData,
        _env: &Env,
    ) {
        if data.capture_screen {
            ctx.submit_command(ENTIRE_SCREEN);
        }
    }

    // Questa funzione serve a definire cosa fare quando si aggiorna il widget (quando si ridisegna)
    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        _old_data: &AppData,
        data: &AppData,
        _env: &Env,
    ) {
        if data.is_dragging == true && data.is_selecting == true {
            ctx.request_paint();
        }
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        _bc: &druid::BoxConstraints,
        _data: &AppData,
        _env: &Env,
    ) -> druid::Size {
        let os = std::env::consts::OS;
        match os {
            "windows" => {
                // Set the size of the drawing area.
                let display_primary = Display::primary().expect("couldn't find primary display");
                //println!("Altezza layout{:?}",display_primary.height());
                let width = display_primary.width();
                let height = display_primary.height();

                ctx.set_paint_insets(druid::Insets::uniform_xy(width as f64, height as f64));
                let size = Size::new(width as f64, height as f64);
                size
            }
            _ => {
                let display_primary = Display::primary().expect("couldn't find primary display");
                //println!("Altezza layout{:?}",display_primary.height());
                let width = display_primary.width();
                let height = display_primary.height();
                ctx.window().set_position(druid::Point::new(0., 0.));
                let size = Size::new(width as f64, height as f64);

                size
            }
        }
    }

    // Questa funzione serve a definire cosa disegnare nel widget e come disegnarlo (con quale colore, con quale font, ecc)
    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &AppData, _env: &Env) {
        if let Some(_start) = data.start_position {
            if let Some(_end) = data.end_position {
                if data.is_selecting == true {
                    let os = env::consts::OS;
                    match os {
                        "windows" => {
                            // disegna il rettangolo selezionato
                            let start_points = data.start_position_to_display.unwrap();
                            let end_points = data.end_position_to_display.unwrap();
                            let rect = druid::Rect::from_points(start_points, end_points);
                            //paint_ctx.fill(rect, &Color::rgba(0.0, 0.0, 1.0, 0.3));
                            paint_ctx.stroke(rect, &Color::RED, 0.9);
                        }
                        _ => {
                            let start_points = data.start_position_to_display.unwrap();
                            let end_points = data.end_position_to_display.unwrap();

                            let rect = druid::Rect::from_points(start_points, end_points);

                            //paint_ctx.fill(rect, &Color::rgba(0.0, 0.0, 1.0, 0.3));
                            paint_ctx.stroke(rect, &Color::RED, 0.9);
                        }
                    }
                }
            }
        }
    }
}
struct MyViewHandler;

//controller per gestire eventi collegati al widget DrawingArea ma gestiti da contesti esterni a drawing_area
impl<W: Widget<AppData>> Controller<AppData, W> for MyViewHandler {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppData,
        env: &Env,
    ) {
        ctx.request_focus();

        match event {
            // Evento che si attiva quando arriva il comando SAVE_FILE_AS
            Event::Command(cmd) if cmd.is(druid::commands::SAVE_FILE_AS) => {
                if let Some(file_info) = cmd.get(druid::commands::SAVE_FILE_AS) {
                    // Salva l'immagine in data.myimage nel percorso specificato
                    data.file_path = file_info.path().to_path_buf().to_str().unwrap().to_string();

                    ctx.set_handled();
                }
            }

            // evento per gestire i tasti premuti
            Event::KeyDown(key_event) => {
                let key;
                if key_event.key != Key::CapsLock {
                    if !data.tasti.contains_key(&key_event.key) {
                        if key_event.key != Key::Control
                            && key_event.key != Key::Shift
                            && key_event.key != Key::Enter
                            && key_event.key != Key::Escape
                        {
                            key = key_event.key.to_string().to_ascii_lowercase();

                            // data.tasti serve a memorizzare i tasti premuti in modo da poterli confrontare con le hotkeys salvate
                            // sistema di doppio buffer per gestire schiaccio e rilascio di una combinazione di tasti
                            data.tasti
                                .insert(Key::Character(key.clone()), Key::Character(key.clone()));
                        } else {
                            data.tasti
                                .insert(key_event.key.clone(), key_event.key.clone());
                        }
                        data.is_found = false;

                        data.count += 1;
                    }
                }
            }

            Event::KeyUp(key_event) => {
                let mut key = key_event.key.clone();
                if key_event.key != Key::CapsLock {
                    if key_event.key != Key::Control
                        && key_event.key != Key::Shift
                        && key_event.key != Key::Enter
                        && key_event.key != Key::Escape
                    {
                        key = Key::Character(key_event.key.to_string().to_ascii_lowercase());
                    }

                    // se il tasto premuto è presente in data.tasti allora lo rimuove da data.tasti e lo aggiunge a data.attivazione
                    //serve a gestire il doppio buffer per gestire schiaccio e rilascio di una combinazione di tasti evitando
                    // ghost key
                    if data.tasti.contains_key(&key) && !data.attivazione.contains_key(&key) {
                        data.attivazione.insert(key.clone(), key.clone());
                        data.tasti.remove(&key);
                        data.count -= 1;
                    }
                    if data.count <= 0 && !data.attivazione.is_empty() {
                        data.count = 0;

                        //verifica se la combinazione di tasti premuti è una delle hotkeys salvate

                        // data.hotkeys.get(i) dove i è l'indice della hotkey salvata in data.hotkeys
                        let mut found = true;

                        //save screen hotkey
                        for key in data.attivazione.keys() {
                            // 0 significa che è la hotkey per salvare l'immagine
                            if !data.hotkeys.get(0).unwrap().keys.contains_key(key)
                                || data.hotkeys.get(0).unwrap().keys.len()
                                    != data.attivazione.keys().len()
                            {
                                found = false;
                                break;
                            }
                        }
                        if found == true {
                            //println!("Save imge hotkey attovata!");
                            // data.hide_buttons = true;
                            data.attivazione.clear();
                            data.is_found = true;
                            if data.myimage.width() != 0 && data.myimage.height() != 0 {
                                screenshot::save_screen_new(data);
                            }

                            data.last_key_event = Some(key_event.clone());
                        }

                        //start hotkeys
                        let mut found = true;
                        if !data.is_found {
                            for key in data.attivazione.keys() {
                                if !data.hotkeys.get(1).unwrap().keys.contains_key(key)
                                    || data.hotkeys.get(1).unwrap().keys.len()
                                        != data.attivazione.keys().len()
                                {
                                    found = false;
                                    break;
                                }
                            }
                            if found == true {
                                data.start_position = None;
                                data.end_position = None;
                                data.start_position_to_display = None;
                                data.end_position_to_display = None;
                                data.is_dragging = false;
                                data.is_selecting = false;
                                data.modify = false;
                                data.rect = Rect::new(0.0, 0.0, 0.0, 0.0);
                                // ctx.request_paint();
                                data.is_found = true;

                                data.attivazione.clear();

                                data.hide_buttons = true;
                                data.last_key_event = Some(key_event.clone());
                            }
                        }

                        // quit hotkey
                        let mut found = true;
                        if !data.is_found {
                            for key in data.attivazione.keys() {
                                if !data.hotkeys.get(2).unwrap().keys.contains_key(key)
                                    || data.hotkeys.get(2).unwrap().keys.len()
                                        != data.attivazione.keys().len()
                                {
                                    found = false;
                                    break;
                                }
                            }
                            if found == true {
                                ctx.submit_command(druid::commands::QUIT_APP);
                            }
                        }
                        //edit hotkey
                        let mut found = true;
                        if !data.is_found {
                            for key in data.attivazione.keys() {
                                if !data.hotkeys.get(3).unwrap().keys.contains_key(key)
                                    || data.hotkeys.get(3).unwrap().keys.len()
                                        != data.attivazione.keys().len()
                                {
                                    found = false;
                                    break;
                                }
                            }
                            if found == true {
                                //sto modificando
                                if data.start_position != None && data.end_position != None {
                                    if let (Some(_start), Some(_end)) =
                                        (data.start_position, data.end_position)
                                    {
                                        // Calculate the selected rectangle
                                        data.is_dragging = true;
                                        data.is_selecting = true;
                                    }
                                    data.is_found = true;
                                    data.hide_buttons = true;

                                    data.attivazione.clear();
                                    data.last_key_event = Some(key_event.clone());
                                    data.is_found = true;
                                }
                            }
                        }

                        //restart from shortkeys - vai alla finestra per scegliere le hotkeys
                        let mut found = true;
                        if !data.is_found {
                            for key in data.attivazione.keys() {
                                if !data.hotkeys.get(4).unwrap().keys.contains_key(key)
                                    || data.hotkeys.get(4).unwrap().keys.len()
                                        != data.attivazione.keys().len()
                                {
                                    found = false;
                                    break;
                                }
                            }
                            if found == true {
                                // data.start_position = None;
                                // data.end_position = None;
                                // data.start_position_to_display = None;
                                // data.end_position_to_display = None;
                                data.is_dragging = false;
                                data.is_selecting = false;
                                data.modify = false;
                                data.hotkeys.clear();

                                data.attivazione.clear();
                                data.is_found = true;
                                data.last_key_event = None;
                                // data.rect = Rect::new(0.0, 0.0, 0.0, 0.0);
                                data.show_drawing = true;
                                let shortkeys_window =
                                    WindowDesc::new(shortkeys_window::ui_builder())
                                        .transparent(false)
                                        .title("Keyboard Shortcut Settings")
                                        .window_size(Size::new(1000., 1000.0))
                                        .set_always_on_top(true)
                                        .show_titlebar(true);
                                ctx.new_window(shortkeys_window);
                                ctx.submit_command(
                                    druid::commands::CLOSE_WINDOW.to(ctx.window_id()),
                                );
                            }
                        }

                        //restart from format hotkey
                        //per scegliere il formato dell'immagine
                        let mut found = true;

                        if !data.is_found {
                            for key in data.attivazione.keys() {
                                if !data.hotkeys.get(5).unwrap().keys.contains_key(key)
                                    || data.hotkeys.get(5).unwrap().keys.len()
                                        != data.attivazione.keys().len()
                                {
                                    found = false;
                                    break;
                                }
                            }
                            if found == true {
                                // data.start_position = None;
                                //     data.end_position = None;
                                //     data.start_position_to_display = None;
                                //     data.end_position_to_display = None;
                                data.is_dragging = false;
                                data.is_selecting = false;
                                data.modify = false;
                                data.is_found = true;
                                data.hide_buttons = false;
                                data.attivazione.clear();

                                data.last_key_event = Some(key_event.clone());
                                // data.rect = Rect::new(0.0, 0.0, 0.0, 0.0);
                                data.is_found = true;
                                let format_window = WindowDesc::new(window_format::build_ui())
                                    .transparent(false)
                                    .title("Choose the format. Default is .png")
                                    .window_size(Size::new(400.0, 400.0))
                                    .set_always_on_top(true)
                                    .show_titlebar(true)
                                    .set_position(Point::new(500., 300.));
                                ctx.new_window(format_window);
                                ctx.submit_command(
                                    druid::commands::CLOSE_WINDOW.to(ctx.window_id()),
                                );
                            }
                        }

                        //clipboard copy hotkey
                        let mut found = true;
                        if !data.is_found {
                            for key in data.attivazione.keys() {
                                if !data.hotkeys.get(6).unwrap().keys.contains_key(key)
                                    || data.hotkeys.get(6).unwrap().keys.len()
                                        != data.attivazione.keys().len()
                                {
                                    found = false;
                                    break;
                                }
                            }
                            if found == true {
                                //data.hotkeys.clear();
                                //println!("Hotkeys clipboard attivata!");
                                data.attivazione.clear();
                                data.is_found = true;
                                data.last_key_event = None;
                                if data.myimage.height() != 0 && data.myimage.width() != 0 {
                                    let clipboard = &mut arboard::Clipboard::new().unwrap();

                                    let bytes = data.myimage.as_bytes();
                                    let img_data = ImageData {
                                        width: data.myimage.width() as usize,
                                        height: data.myimage.height() as usize,
                                        bytes: bytes.as_ref().into(),
                                    };
                                    clipboard.set_image(img_data).unwrap();
                                }
                            }
                        }

                        //entire screen capture

                        let mut found = true;
                        if !data.is_found {
                            for key in data.attivazione.keys() {
                                if !data.hotkeys.get(7).unwrap().keys.contains_key(key)
                                    || data.hotkeys.get(7).unwrap().keys.len()
                                        != data.attivazione.keys().len()
                                {
                                    found = false;
                                    break;
                                }
                            }
                            if found == true {
                                data.hide_buttons = true;
                                data.capture_screen = true;
                                data.end_position = None;
                                data.end_position_to_display = None;
                                data.start_position_to_display = None;
                                data.start_position = None;
                                data.is_dragging = false;
                                data.is_selecting = false;
                                data.attivazione.clear();
                                data.last_key_event = Some(key_event.clone());
                                data.is_found = true;
                            }
                        }

                        data.attivazione.clear();
                    }
                    data.count = 0;
                }
            }
            _ => {}
        }
        child.event(ctx, event, data, env);
    }
}

pub(crate) fn build_ui() -> impl Widget<AppData> {
    let dimensioni = Display::primary().expect("error");
    let skip_panel = ViewSwitcher::new(
        move |data: &AppData, _env| data.hide_buttons,
        move |selector, data: &AppData, _env| {
            let mut s = "".to_string();
            let mut color_border = Color::WHITE;
            if data.myimage.width() == 0 && data.myimage.height() == 0 {
                color_border = Color::TRANSPARENT;
            }
            if data.start_position == data.end_position
                && data.start_position != None
                && data.end_position != None
                && data.start_position != Some(Point::new(0., 0.))
                && data.end_position != Some(Point::new(0., 0.))
            {
                s = "You pressed only one point on the screen, invalid area to perform a capture "
                    .to_string();
            }
            match selector {
                false => Box::new(
                    Box::new(
                        Flex::column()
                            .with_child(
                                Flex::row()
                                    .with_child(Padding::new(
                                        Insets::new(40., 40., 1., 40.),
                                        Button::new("Start").on_click(
                                            |_: &mut EventCtx, data: &mut AppData, _: &Env| {
                                                data.hide_buttons = true;
                                                data.end_position = None;
                                                data.end_position_to_display = None;
                                                data.start_position_to_display = None;
                                                data.start_position = None;
                                                data.is_dragging = false;
                                                data.is_selecting = false;
                                            },
                                        ),
                                    ))
                                    .with_child(Button::new("Entire screen").on_click(
                                        |_ctx: &mut EventCtx, data: &mut AppData, _: &Env| {
                                            data.hide_buttons = true;
                                            data.capture_screen = true;
                                            data.end_position = None;
                                            data.end_position_to_display = None;
                                            data.start_position_to_display = None;
                                            data.start_position = None;
                                            data.is_dragging = false;
                                            data.is_selecting = false;
                                        },
                                    ))
                                    .with_child(Button::new("Save Screen").on_click(
                                        |_ctx: &mut EventCtx, data: &mut AppData, _env: &Env| {
                                            if data.myimage.width() != 0
                                                && data.myimage.height() != 0
                                            {
                                                screenshot::save_screen_new(data);
                                            }
                                        },
                                    ))
                                    .with_child(Button::new("Edit").on_click(
                                        |_ctx: &mut EventCtx, data: &mut AppData, _: &Env| {
                                            if data.start_position != None
                                                && data.end_position != None
                                                && data.start_position != data.end_position
                                            {
                                                data.hide_buttons = true;
                                                data.is_dragging = true;
                                                data.is_selecting = true;
                                            }
                                        },
                                    ))
                                    .with_child(Button::new("Copy to clipboard").on_click(
                                        |_ctx: &mut EventCtx, data: &mut AppData, _: &Env| {
                                            if data.myimage.height() != 0
                                                && data.myimage.width() != 0
                                            {
                                                let clipboard =
                                                    &mut arboard::Clipboard::new().unwrap();

                                                let bytes = data.myimage.as_bytes();
                                                let img_data = ImageData {
                                                    width: data.myimage.width() as usize,
                                                    height: data.myimage.height() as usize,
                                                    bytes: bytes.as_ref().into(),
                                                };
                                                clipboard.set_image(img_data).unwrap();
                                            }
                                        },
                                    ))
                                    .with_child(Button::new("Close").on_click(
                                        |ctx: &mut EventCtx, _data: &mut AppData, _: &Env| {
                                            ctx.submit_command(druid::commands::QUIT_APP);
                                        },
                                    ))
                                    .with_child(Button::new("Choose your shortkeys").on_click(
                                        |ctx: &mut EventCtx, data: &mut AppData, _: &Env| {
                                            data.is_dragging = false;
                                            data.is_selecting = false;
                                            data.modify = false;
                                            data.hotkeys.clear();
                                            data.is_found = false;
                                            data.last_key_event = None;
                                            data.show_drawing = true;

                                            let shortkeys_window =
                                                WindowDesc::new(shortkeys_window::ui_builder())
                                                    .transparent(false)
                                                    .title("Keyboard Shortcut Settings")
                                                    .window_size(Size::new(1000., 1000.0))
                                                    .set_always_on_top(true)
                                                    .show_titlebar(true);
                                            ctx.new_window(shortkeys_window);
                                            ctx.submit_command(
                                                druid::commands::CLOSE_WINDOW.to(ctx.window_id()),
                                            );
                                        },
                                    ))
                                    .with_child(Button::new("Choose image format").on_click(
                                        |ctx: &mut EventCtx, data: &mut AppData, _: &Env| {
                                            data.is_dragging = false;
                                            data.is_selecting = false;
                                            data.modify = false;
                                            data.is_found = false;
                                            data.last_key_event = None;

                                            ctx.submit_command(
                                                druid::commands::CLOSE_WINDOW.to(ctx.window_id()),
                                            );
                                            let format_window =
                                                WindowDesc::new(window_format::build_ui())
                                                    .transparent(false)
                                                    .title("Choose the format. Default is .png")
                                                    .window_size(Size::new(400.0, 400.0))
                                                    .set_always_on_top(true)
                                                    .show_titlebar(true)
                                                    .set_position(Point::new(500., 300.));
                                            ctx.new_window(format_window);
                                        },
                                    ))
                                    .with_child(
                                        Button::new("Choose image path for savings").on_click(
                                            |ctx: &mut EventCtx, _data: &mut AppData, _: &Env| {
                                                let file_options = FileDialogOptions::new()
                                                    .default_name("screenshot_grabbed");

                                                ctx.submit_command(
                                                    druid::commands::SHOW_SAVE_PANEL
                                                        .with(file_options),
                                                );
                                            },
                                        ),
                                    )
                                    .with_child(Button::new("Choose name convention").on_click(
                                        |ctx: &mut EventCtx, data: &mut AppData, _: &Env| {
                                            data.is_dragging = false;
                                            data.is_selecting = false;
                                            data.modify = false;
                                            data.is_found = false;
                                            data.last_key_event = None;

                                            ctx.submit_command(
                                                druid::commands::CLOSE_WINDOW.to(ctx.window_id()),
                                            );
                                            let convention_window =
                                                WindowDesc::new(convention_window::build_ui())
                                                    .transparent(false)
                                                    .title("Choose the convention")
                                                    .window_size(Size::new(200.0, 200.0))
                                                    .set_always_on_top(true)
                                                    .show_titlebar(true);
                                            ctx.new_window(convention_window);
                                        },
                                    ))
                                    .with_child(Button::new("?").on_click(
                                        |ctx: &mut EventCtx, _data: &mut AppData, _: &Env| {
                                            ctx.submit_command(
                                                druid::commands::CLOSE_WINDOW.to(ctx.window_id()),
                                            );
                                            let information_window =
                                                WindowDesc::new(information_window::build_ui())
                                                    .transparent(false)
                                                    .title("Instructions")
                                                    .window_size(Size::new(1000.0, 1000.0))
                                                    .set_always_on_top(true)
                                                    .show_titlebar(true)
                                                    .resizable(false);
                                            ctx.new_window(information_window);
                                        },
                                    )),
                            )
                            .with_child(Label::new(
                                "To exit edit mode, click on any position of the screen",
                            ))
                            .with_child(Label::new(s))
                            .with_child(
                                // questo mostra l'immagine catturata se è presente altrimenti mostra un rettangolo vuoto
                                Image::new(ImageBuf::from_raw(
                                    data.myimage.clone().into_raw(),
                                    ImageFormat::RgbaSeparate,
                                    data.myimage.width() as usize,
                                    data.myimage.height() as usize,
                                ))
                                .center()
                                .fix_size(
                                    dimensioni.width() as f64 / 1.8 as f64,
                                    dimensioni.height() as f64 / 1.8 as f64,
                                )
                                .center()
                                .border(color_border, 1.0),
                            ),
                    )
                    .fix_size(
                        Display::primary().expect("erro").width() as f64,
                        Display::primary().expect("erro").height() as f64,
                    )
                    //imposto sfondo viola
                    .background(BackgroundBrush::Color(Color::rgba(
                        60. / 255.,
                        8. / 255.,
                        120. / 255.,
                        1.,
                    ))),
                ),
                true => Box::new(
                    Flex::column()
                        .with_child(DrawingArea)
                        .background(BackgroundBrush::Color(Color::BLACK.with_alpha(0.08))),
                ),
            }
        },
    );

    Flex::column()
        .with_child(skip_panel)
        .controller(MyViewHandler)
}
