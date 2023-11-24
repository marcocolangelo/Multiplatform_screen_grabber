use chrono::Local;
use druid::{MouseEvent, Point};

use crate::drawing_area::{self, AppData, DragHandle};
pub(crate) fn default_convention(path: String) -> String {
    let new_path = path;
    new_path
}
pub(crate) fn time_convention(path: String) -> String {
    let now = Local::now().format("%d-%m-%Y_%H-%M").to_string();
    let new_path = path + "_" + &now;
    new_path
}
pub(crate) fn numeric_convention(path: String, data: &mut AppData) -> String {
    data.counter += 1;
    let new_path = path + "_" + &data.counter.to_string();
    new_path
}
pub(crate) fn some_fields_are_equal(data: &drawing_area::AppData) -> bool {
    if (data.start_image_modifier == data.save_image_modifier
        && data.start_image_key == data.save_image_key)
        || (data.start_image_modifier == data.restart_app_modifier
            && data.start_image_key == data.restart_app_key)
        || (data.start_image_modifier == data.copy_clipboard_modifier
            && data.start_image_key == data.copy_clipboard_key)
        || (data.start_image_modifier == data.restart_format_app_modifier
            && data.start_image_key == data.restart_format_app_key)
        || (data.start_image_modifier == data.edit_image_modifier
            && data.start_image_key == data.edit_image_key)
        || (data.start_image_modifier == data.quit_app_modifier
            && data.start_image_key == data.quit_app_modifier)
        || (data.save_image_modifier == data.quit_app_modifier
            && data.save_image_key == data.quit_app_key)
        || (data.save_image_modifier == data.copy_clipboard_modifier
            && data.save_image_key == data.copy_clipboard_key)
        || (data.save_image_modifier == data.restart_format_app_modifier
            && data.save_image_key == data.restart_format_app_key)
        || (data.save_image_modifier == data.restart_app_modifier
            && data.save_image_key == data.restart_app_key)
        || (data.save_image_modifier == data.edit_image_modifier
            && data.save_image_key == data.edit_image_key)
        || (data.quit_app_modifier == data.edit_image_modifier
            && data.quit_app_key == data.edit_image_key)
        || (data.quit_app_modifier == data.copy_clipboard_modifier
            && data.quit_app_key == data.copy_clipboard_key)
        || (data.quit_app_modifier == data.restart_app_modifier
            && data.quit_app_key == data.restart_app_key)
        || (data.quit_app_modifier == data.restart_format_app_modifier
            && data.quit_app_key == data.restart_format_app_key)
        || (data.edit_image_modifier == data.restart_app_modifier
            && data.edit_image_key == data.restart_app_key)
        || (data.edit_image_modifier == data.copy_clipboard_modifier
            && data.edit_image_key == data.copy_clipboard_key)
        || (data.restart_app_modifier == data.copy_clipboard_modifier
            && data.restart_app_key == data.copy_clipboard_key)
        || (data.restart_app_modifier == data.restart_format_app_modifier
            && data.restart_app_key == data.restart_format_app_key)
        || (data.edit_image_modifier == data.restart_format_app_modifier
            && data.edit_image_key == data.restart_format_app_key)
        || (data.copy_clipboard_modifier == data.restart_format_app_modifier
            && data.copy_clipboard_key == data.restart_format_app_key)
    {
        true
    } else {
        false
    }
}

pub(crate) fn are_all_fields_completed(data: &drawing_area::AppData) -> bool {
    if (data.save_image_modifier != "None".to_string() || data.save_image_key != "".to_string())
        && (data.edit_image_modifier != "None".to_string() || data.edit_image_key != "".to_string())
        && (data.quit_app_key != "".to_string() || data.quit_app_modifier != "None".to_string())
        && (data.start_image_key != "".to_string()
            || data.start_image_modifier != "None".to_string())
        && (data.restart_app_modifier != "None".to_string()
            || data.restart_app_key != "".to_string())
        && (data.restart_format_app_modifier != "None".to_string()
            || data.restart_format_app_key != "".to_string())
        && (data.copy_clipboard_modifier != "None".to_string()
            || data.copy_clipboard_key != "".to_string())
    {
        true
    } else {
        false
    }
}

pub(crate) fn edit_rect(
    handle: &DragHandle,
    pos: Point,
    data: &mut AppData,
    mouse_event: &MouseEvent,
) {
    match handle {
        DragHandle::TopLeft => {
            data.rect.x0 = mouse_event.pos.x;
            data.rect.y0 = mouse_event.pos.y;
            // let pos = ctx.to_screen(druid::Point::new(mouse_event.pos.x, mouse_event.pos.y));

            let coord = druid::Point { x: pos.x, y: pos.y };
            data.start_position_to_display = Some(druid::Point {
                x: mouse_event.pos.x,
                y: mouse_event.pos.y,
            });
            data.start_position = Some(coord);
            data.is_selecting = true;

            //println!("{:?}, {:?}",data.start_position,data.end_position);
        }
        DragHandle::BottomRight => {
            data.rect.x1 = mouse_event.pos.x;
            data.rect.y1 = mouse_event.pos.y;
            // let pos = ctx.to_screen(druid::Point::new(mouse_event.pos.x, mouse_event.pos.y));

            let coord = druid::Point { x: pos.x, y: pos.y };
            data.end_position_to_display = Some(druid::Point {
                x: mouse_event.pos.x,
                y: mouse_event.pos.y,
            });
            data.end_position = Some(coord);
            data.is_selecting = true;
        }
        DragHandle::BottomLeft => {
            data.rect.x0 = mouse_event.pos.x;
            data.rect.y1 = mouse_event.pos.y;
            // let pos = ctx.to_screen(druid::Point::new(mouse_event.pos.x, mouse_event.pos.y));

            let coord = druid::Point {
                x: data.end_position.unwrap().x,
                y: pos.y,
            };
            data.end_position_to_display = Some(druid::Point {
                x: data.end_position_to_display.unwrap().x,
                y: mouse_event.pos.y,
            });
            data.end_position = Some(coord);
            let coord = druid::Point {
                x: pos.x,
                y: data.start_position.unwrap().y,
            };
            data.start_position_to_display = Some(druid::Point {
                x: data.rect.x0,
                y: data.start_position_to_display.unwrap().y,
            });
            data.start_position = Some(coord);
            data.is_selecting = true;
        }
        DragHandle::TopRight => {
            data.rect.x1 = mouse_event.pos.x;
            data.rect.y0 = mouse_event.pos.y;
            // let pos = ctx.to_screen(druid::Point::new(mouse_event.pos.x, mouse_event.pos.y));

            let coord = druid::Point {
                x: pos.x,
                y: data.end_position.unwrap().y,
            };
            data.end_position_to_display = Some(druid::Point {
                x: mouse_event.pos.x,
                y: data.end_position_to_display.unwrap().y,
            });
            data.end_position = Some(coord);
            let coord = druid::Point {
                x: data.start_position.unwrap().x,
                y: pos.y,
            };

            data.start_position_to_display = Some(druid::Point {
                x: data.start_position_to_display.unwrap().x,
                y: data.rect.y0,
            });
            data.start_position = Some(coord);
            data.is_selecting = true;
        }
    }
}
