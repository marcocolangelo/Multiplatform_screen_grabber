use druid::{
    text::FontDescriptor,
    widget::{Align, Button, Controller, Flex, Label},
     Size, Widget, WidgetExt, WindowDesc, Insets,
};

use druid_shell::piet::{FontFamily, FontWeight};
use scrap::Display;

use crate::drawing_area::{self, AppData};

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
                    let display_primary = Display::primary().expect("couldn't find primary display");

                    let main_window = WindowDesc::new(drawing_area::build_ui())
                        .show_titlebar(false)
                        .set_position(druid::Point::new(0., 0.))
                        .window_size(Size::new(
                            display_primary.width() as f64,
                            display_primary.height() as f64,
                        ))
                        .resizable(true)
                        //.show_titlebar(false)
                        .transparent(true)
                        .set_window_state(druid_shell::WindowState::Maximized);

                    // let id = main_window.id.clone();
                    ctx.new_window(main_window);
                    data.switch_window = true;
                    // ctx.submit_command(druid::commands::SHOW_WINDOW.to(id));

                    ctx.submit_command(druid::commands::CLOSE_WINDOW.to(ctx.window_id()));
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
pub(crate) fn build_ui() -> impl Widget<AppData> {
    let button = Button::new("Return").on_click(move |ctx, data: &mut AppData, _| {
        let display_primary = Display::primary().expect("couldn't find primary display");

        let main_window = WindowDesc::new(drawing_area::build_ui())
            .show_titlebar(false)
            .set_position(druid::Point::new(0., 0.))
            .window_size(Size::new(
                display_primary.width() as f64,
                display_primary.height() as f64,
            ))
            .resizable(true)
            //.show_titlebar(false)
            .transparent(true)
            .set_window_state(druid_shell::WindowState::Maximized);

        // let id = main_window.id.clone();
        ctx.new_window(main_window);
        data.switch_window = true;
        // ctx.submit_command(druid::commands::SHOW_WINDOW.to(id));

        ctx.submit_command(druid::commands::CLOSE_WINDOW.to(ctx.window_id()));
        ctx.set_handled();
    });

    // let textbox = TextBox::new()
    //     .with_placeholder("choose the name of the screen (default screenshot_grabbed)")
    //     .lens(AppData::label)
    //     .padding(3.0);
    let mut column = Flex::column();

    let instructions = vec![
        ("Start: ", "Pressing this button you are able to make the user interface disappear and capture the screen. \nClicking and dragging the mouse pointer you can draw the area to capture,\notherwise just clicking on the screen you can capture the whole monitor"),
        ("Save screen: ", "Pressing this button you can save your captured image, \ndefault path is the same path as the executable of the program "),
        ("Close: ", "Pressing this button the entire application it's closed."),
        ("Edit: ", "Pressing this button you are able to edit your capture area, by clicking on one of the drawn rectangle corners and then \ndragging the clicked corner. To stop your editing step you can click on any other point of the screen"),
        ("Choose your shortkeys: ", "Pressing this button you can set up your favorite shortkeys related to most of the features"),
        ("Choose image format: ", "Pressing this button, you can select the format the image must be saved with"),
        ("Copy to clipboard: ", "Pressing this button, you are able to copy the just captured image to the clipboard"),
        ("Choose image path for savings", "Pressing this button, you can decide the path the image must be saved with"),
        ("Choose name convention","Pressing this button, you can choose the name convention the image must be saved with "),
        
    ];

    for (instruction, description) in instructions {
        let instruction_label = Label::new(instruction)
            .with_font(
                FontDescriptor::new(FontFamily::SYSTEM_UI)
                    .with_weight(FontWeight::BOLD)
                    .with_size(16.0),
            )
            ;
        let description_label = Label::new(description)
            .with_font(FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(16.0))
            ;

        let row = Flex::row()
            .with_child(Align::centered(instruction_label))
            .with_default_spacer()
            .with_child(Align::centered(description_label))
            // .with_spacer(f64::INFINITY)
            .padding(Insets::new(20., 20., 20., 20.))
            ;

        column= column.with_child(row);
    }
    return column.with_child(button) .controller(MyViewHandler);

    // Flex::column()
    //     .with_child(
    //         Flex::row().with_child(
    //         Label::new("Start:").with_font(
    //             FontDescriptor::new(FontFamily::SYSTEM_UI).with_weight(FontWeight::BOLD),
    //         ),
    //     )
    //     .with_child(Label::new("pressing this button you are able to make the user interface disappear and capture the screen.\n
    //     Clicking and dragging the mouse pointer you can draw the area to capture,\n
    //     otherwise just clicking on the screen you can capture the whole monitor\n
    //     ")))
    // .with_child(button)
    // .controller(MyViewHandler)
}
