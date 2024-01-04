use druid::{
    widget::{Button, Controller, Flex, Label, RadioGroup},
    Size, Widget, WidgetExt, WindowDesc,
};

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
pub(crate) fn build_ui() -> impl Widget<AppData> {
    let button = Button::new("Save").on_click(move |ctx, data: &mut AppData, _| {
        let display_primary = Display::primary().expect("couldn't find primary display");

        let main_window = WindowDesc::new(drawing_area::build_ui())
            .show_titlebar(false)
            .set_position(druid::Point::new(0., 0.))
            .window_size(Size::new(
                display_primary.width() as f64,
                display_primary.height() as f64,
            ))
            .resizable(true)
            .transparent(true)
            .set_window_state(druid_shell::WindowState::Maximized);

        ctx.new_window(main_window);
        data.switch_window = true;

        ctx.submit_command(druid::commands::CLOSE_WINDOW.to(ctx.window_id()));
        ctx.set_handled();
    });

    Flex::column()
        .with_child(Label::new("Default name: screenshot_grabbed"))
        .with_child(
            RadioGroup::column(vec![
                (
                    "Default convention",
                    drawing_area::Conventions::DefaultConvention,
                ),
                (
                    "Time convention ",
                    drawing_area::Conventions::TimeConvention,
                ),
                (
                    "Numeric convention ",
                    drawing_area::Conventions::NumericConvention,
                ),
            ])
            .lens(AppData::my_convention),
        )
        .with_child(button)
        .controller(MyViewHandler)
}
