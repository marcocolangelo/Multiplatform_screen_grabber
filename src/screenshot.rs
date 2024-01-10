use druid::Point;
use image::{ImageBuffer, Rgba};

use screenshots::Screen;

use crate::{
    drawing_area::{self, AppData},
    function,
    window_format::MyRadio,
};

// serve a effettruare uno screenshot della schermata attuale
pub(crate) fn screen_new(
    start_position: Point,
    end_position: Point,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let screen = Screen::from_point(0, 0).unwrap();

    let mut end_y = end_position.y;
    let mut end_x = end_position.x;
    let mut start_x = start_position.x;
    let mut start_y = start_position.y;
    if start_x > end_x {
        let prov_x0 = start_x;
        start_x = end_x;
        end_x = prov_x0;
    }
    if start_y > end_y {
        let prov_y0 = start_y;
        start_y = end_y;
        end_y = prov_y0;
    }
    end_y = end_y - 2.1;
    end_x = end_x - 2.1;
    start_x = start_x + 2.1;
    start_y = start_y + 2.1;

    //elimina la cornice
    let mut width = (end_x - start_x) - 1.;
    let mut height = end_y - start_y - 1.;

    // se troppo piccolo lo rendo 1 pixel
    if width < 1. {
        width = 1.;
    }
    if height < 1. {
        height = 1.;
    }

    let image = screen
        .capture_area(start_x as i32, start_y as i32, width as u32, height as u32)
        .unwrap();
    return image;
}

// serve a salvare lo screenshot appena effettuato in un file secondo la convenzione scelta e il formato scelto
pub(crate) fn save_screen_new(data: &mut AppData) {
    let new_format = data.radio_group;

    let form = match new_format {
        MyRadio::Png => "png",
        MyRadio::Jpeg => "jpeg",
        MyRadio::Gif => "gif",
    };
    let myimage = data.myimage.clone();

    let new_path = match data.my_convention {
        drawing_area::Conventions::DefaultConvention => {
            function::default_convention(data.file_path.clone())
        }
        drawing_area::Conventions::TimeConvention => {
            function::time_convention(data.file_path.clone())
        }
        drawing_area::Conventions::NumericConvention => {
            function::numeric_convention(data.file_path.clone(), data)
        }
    };
    std::thread::spawn(move || {
        myimage.save(new_path + "." + form).unwrap();
    });
}
