extern crate image;

use druid::{AppLauncher, Color, Command, Target, Widget, WidgetExt, WindowDesc};
use druid::FileDialogOptions;
use druid::widget::{Button, FillStrat, Flex};

use app_state::AppState;
use components::gallery::Gallery;
use components::message_box::MessageBox;
use sit_delegate::{DONE, OPENING, PROCESSING, SitDelegate, WATERMARK};

const LIGHTER_GREY: Color = Color::rgb8(242, 242, 242);

pub mod app_state;
pub mod sit_delegate;
pub mod components;
pub mod sit_menu;
pub mod sit_image;

fn button() -> impl Widget<AppState> {
    Flex::row()
        .with_child(Button::new("打开文件").on_click(|ctx, _data: &mut AppState, _env| {
            ctx.submit_command(OPENING);
            ctx.submit_command(Command::new(
                druid::commands::SHOW_OPEN_PANEL,
                FileDialogOptions::new(),
                Target::Auto,
            ))
        }))
        .with_default_spacer()
        .with_child(Button::new("设置水印").on_click(|ctx, _data: &mut AppState, _env| {
            ctx.submit_command(WATERMARK);
            ctx.submit_command(Command::new(
                druid::commands::SHOW_OPEN_PANEL,
                FileDialogOptions::new(),
                Target::Auto,
            ));
        }))
        .with_default_spacer()
        .with_child(
            Button::new("转换").on_click(|ctx, data: &mut AppState, _env| {
                ctx.submit_command(PROCESSING);
                data.process_files(ctx);
                ctx.submit_command(DONE);
            })
        )
}

fn make_ui() -> impl Widget<AppState> {
    let flex = Flex::column();
    flex.with_child(Gallery::new())
        .with_default_spacer()
        .with_child(button())
        .with_default_spacer()
        .with_child(MessageBox::new())
        .background(LIGHTER_GREY)
}

const IMAGE_MAX_WIDTH: u32 = 3072;

pub fn main() {
    let title = "Hug8217";

    let main_window = WindowDesc::new(make_ui())
        .window_size((512., 384.))
        .with_min_size((512., 384.))
        .menu(sit_menu::make_menu)
        .title(title);

    let init_state = AppState {
        fill_strat: FillStrat::Cover,
        title: "".to_string(),
        files: vec![],
        watermark: "".to_string(),
        messages: vec![],
        status: "".to_string(),
    };

    AppLauncher::with_window(main_window)
        .delegate(SitDelegate::default())
        .log_to_console()
        .launch(init_state)
        .expect("Failed to launch application");
}
