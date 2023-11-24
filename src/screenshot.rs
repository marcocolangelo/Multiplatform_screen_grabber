use druid::Point;
use image::{ImageBuffer, Rgba};

use screenshots::Screen;

use crate::{
    drawing_area::{self, AppData},
    function,
    window_format::MyRadio,
};

pub(crate) fn screen_new(
    start_position: Point,
    end_position: Point,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let screen = Screen::from_point(0, 0).unwrap();

    if start_position != end_position {
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
        let mut width = (end_x - start_x) - 1.;
        let mut height = end_y - start_y - 1.;
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
    } else {
        let image = screen.capture().unwrap();
        return image;
    }
}

pub(crate) fn save_screen_new(data: &mut AppData) {
    let new_format = data.radio_group;
    // let name_capture = data.label.clone();
    let form = match new_format {
        MyRadio::Png => "png",
        MyRadio::Jpeg => "jpeg",
        MyRadio::Gif => "gif",
    };
    let myimage = data.myimage.clone();
    // println!("image: {:?}", myimage.width());

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

// pub fn screen(
//     format: MyRadio,
//     start_position: Arc<Mutex<Option<(f64, f64)>>>,
//     end_position: Arc<Mutex<Option<(f64, f64)>>>,
//     name: String,
//     save: bool,
// ) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
//     let new_format = format;
//     let name_capture = name;
//     let mut image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(0, 0);
//     // altri sistemi operativi è l'unico che non da problemi di distorsione, controllare se su windows funziona ugualmente e in tal caso sostituire
//     if let (Some((x1, y1)), Some((x2, y2))) = (
//         *start_position.lock().unwrap(),
//         *end_position.lock().unwrap(),
//     ) {
//         let form = match new_format {
//             MyRadio::Png => "png",
//             MyRadio::Jpeg => "jpeg",
//             MyRadio::Bmp => "bmp",
//         };
//         let screen = Screen::from_point(0, 0).unwrap();

//         image = screen
//             .capture_area(
//                 x1 as i32 + 1,
//                 y1 as i32 + 1,
//                 ((x2 - x1) - 1.5) as u32,
//                 ((y2 - y1) - 1.5) as u32,
//             )
//             .unwrap();
//         if save {
//             image
//                 .save(name_capture.to_owned() + "." + form.clone())
//                 .unwrap();
//             // let name = name_capture.to_owned() + "." + form.clone();
//             let clipboard = &mut arboard::Clipboard::new().unwrap();

//             let bytes = image.as_bytes();
//             let img_data = ImageData {
//                 width: image.width() as usize,
//                 height: image.height() as usize,
//                 bytes: bytes.as_ref().into(),
//             };
//             clipboard.set_image(img_data).unwrap();
//         }
//     }
//     image
// }

// // Versione che fa post processing in un thread apparte. Al momento non funzionante
// pub fn screen_thread( _is_dragging: Arc<Mutex<Option<bool>>>,mut capturer:Capturer,width: u32,height: u32,start_position:Arc<Mutex<Option<(f64, f64)>>>,end_position:Arc<Mutex<Option<(f64, f64)>>>)
// {

//     // Aspetta finché l'utente non ha finito di tracciare l'area dello schermo.

//     let frame: Option<Vec<u8>> = loop {
//         match capturer.frame() {

//             /* DOPO L'AGGIUNTA DELLO STRIDE GLI SCREEN SONO SPOSTATI LEGGERMENTE IN ALTO, VEDI DI AGGIUSTARE*/
//             Ok(frame) => {
//                 // println!("Demtro frame");
//                 // Calcola lo stride dell'immagine
//                 let stride = frame.len() / height as usize;

//                 // Crea una nuova immagine con le dimensioni corrette
//                 let mut image = vec![0; stride * height as usize];

//                 // Copia i dati dell'immagine, tenendo conto dello stride
//                 for y in 0..height {
//                     let src_off = (y * stride as u32) as usize;
//                     let dst_off = (y * width * 4) as usize;
//                     let src_slice = &frame[src_off..src_off + width as usize * 4];
//                     let dst_slice = &mut image[dst_off..dst_off + width as usize * 4];
//                     dst_slice.copy_from_slice(src_slice);
//                 }
//                     // Ritorna l'immagine modificata
//                     break Some(image);
//                 }
//                     ,
//             Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
//                 // Aspetta un po' per il prossimo frame.

//                 thread::sleep(Duration::from_millis(100));
//                 break None;
//             }
//             Err(e) => panic!("frame error: {}", e),
//         }
//     };

//     if frame.is_none(){
//         return;
//     }

//     let frame = frame.unwrap();

//     thread::spawn(move ||{
//                     // Converte il frame in un'immagine.
//                 let mut image: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(
//                     width as u32,
//                     height as u32,
//                     frame.to_vec(),
//                 ).unwrap();

//                 for pixel in image.pixels_mut() {
//                     let b = pixel[0];
//                     let g = pixel[1];
//                     let r = pixel[2];
//                     let a = pixel[3];
//                     *pixel = Rgba([r, g, b, a]);
//                 }

//             // println!("height: {:?}",height);
//             // // Converte il frame in un'immagine.
//             // let mut image: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(
//             //     width as u32,
//             //     height as u32,
//             //     frame.to_vec(),
//             // ).unwrap();

//             // for pixel in image.pixels_mut() {
//             //     let b = pixel[0];
//             //     let g = pixel[1];
//             //     let r = pixel[2];
//             //     let a = pixel[3];
//             //     *pixel = Rgba([r, g, b, a]);
//             // }

//             let im=DynamicImage::from(image.clone());
//             let _=DynamicImage::from(im).save("original.png");
//             //println!("Image size: {:?}, {:?}", im.width(), im.height());
//             // Ritaglia l'immagine all'area specificata.
//             if let (Some((x1, y1)), Some((x2, y2))) = (*start_position.lock().unwrap(), *end_position.lock().unwrap()) {

//                 let sub_image = (DynamicImage::from(image)).crop((x1+1.) as u32, (y1+1.) as u32, (x2-x1-1.5) as u32, (y2-y1-1.5) as u32);
//                 //println!("{:?}, {:?} x1: {:?}, y1: {:?}", (x2-x1), (y2-y1),x1,y1);
//                 match sub_image.save("screenshot_grabbed.png") {
//                     Ok(_) => {println!("Successo!");return},
//                     Err(e) if e.to_string().contains("Zero width not allowed") => {println!("Errore");return},
//                     Err(_) => panic!("Unexpected error!"),
//                 }

//             }
//     });

// }

// // Versione senza mutex, potrebbe essere rischiosa da usare
// pub fn screen_no_mutex( _is_dragging:Option<bool>,mut capturer:Capturer,width: u32,height: u32,start_position:Option<(f64, f64)>,end_position:Option<(f64, f64)>)
// {

//  loop{

//     // Aspetta finché l'utente non ha finito di tracciare l'area dello schermo.

//     let frame: Option<Vec<u8>> = loop {
//         match capturer.frame() {

//             /* DOPO L'AGGIUNTA DELLO STRIDE GLI SCREEN SONO SPOSTATI LEGGERMENTE IN ALTO, VEDI DI AGGIUSTARE*/
//             Ok(frame) => {
//                 // println!("Demtro frame");
//                 // Calcola lo stride dell'immagine
//                 let stride = frame.len() / height as usize;

//                 // Crea una nuova immagine con le dimensioni corrette
//                 let mut image = vec![0; stride * height as usize];

//                 // Copia i dati dell'immagine, tenendo conto dello stride
//                 for y in 0..height {
//                     let src_off = (y * stride as u32) as usize;
//                     let dst_off = (y * width * 4) as usize;
//                     let src_slice = &frame[src_off..src_off + width as usize * 4];
//                     let dst_slice = &mut image[dst_off..dst_off + width as usize * 4];
//                     dst_slice.copy_from_slice(src_slice);
//                 }
//                     // Ritorna l'immagine modificata
//                     break Some(image);
//                 }
//                     ,
//             Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
//                 // Aspetta un po' per il prossimo frame.

//                 thread::sleep(Duration::from_millis(100));
//                 break None;
//             }
//             Err(e) => panic!("frame error: {}", e),
//         }
//     };

//     if frame.is_none(){
//         continue;
//     }

//     let frame = frame.unwrap();

//      // Converte il frame in un'immagine.
//      let mut image: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(
//          width as u32,
//          height as u32,
//          frame.to_vec(),
//      ).unwrap();

//      for pixel in image.pixels_mut() {
//          let b = pixel[0];
//          let g = pixel[1];
//          let r = pixel[2];
//          let a = pixel[3];
//          *pixel = Rgba([r, g, b, a]);
//      }

//     let im=DynamicImage::from(image.clone());
//     let _=DynamicImage::from(im).save("original.png");
//     //println!("Image size: {:?}, {:?}", im.width(), im.height());
//     // Ritaglia l'immagine all'area specificata.
//     if let (Some((x1, y1)), Some((x2, y2))) = (start_position, end_position) {
//         let sub_image = (DynamicImage::from(image)).crop((x1) as u32, (y1) as u32, (x2-x1) as u32, (y2-y1) as u32);
//         //println!("{:?}, {:?} x1: {:?}, y1: {:?}", (x2-x1), (y2-y1),x1,y1);
//         match sub_image.save("screenshot_grabbed.png") {
//             Ok(_) => break,
//             Err(e) if e.to_string().contains("Zero width not allowed") => continue,
//             Err(_) => panic!("Unexpected error!"),
//         }

//     }
// }
// }
