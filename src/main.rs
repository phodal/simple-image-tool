extern crate image;

use std::fs::File;
use std::path::Path;

use druid::{AppLauncher, Color, Command, Env, Menu, Target, Widget, WidgetExt, WindowDesc, WindowId};
use druid::{
    commands, Data, FileDialogOptions, LocalizedString, MenuItem, platform_menus, SysMods,
};
use druid::widget::{Button, FillStrat, Flex};
use image::{GenericImageView, ImageFormat};
use image::imageops::FilterType;

use app_state::AppState;
use components::gallery::Gallery;
use components::message_box::MessageBox;
use sit_delegate::{DONE, MESSAGE, OPENING, PROCESSING, SitDelegate, WATERMARK};

const LIGHTER_GREY: Color = Color::rgb8(242, 242, 242);

pub mod app_state;
pub mod sit_delegate;
pub mod components;

fn make_menu(_: Option<WindowId>, _state: &AppState, _: &Env) -> Menu<AppState> {
    let mut menu = Menu::empty();
    #[cfg(target_os = "macos")]
        {
            menu = menu.entry(platform_menus::mac::application::default());
        }

    menu.entry(file_menu())
}

fn file_menu<T: Data>() -> Menu<T> {
    Menu::new(LocalizedString::new("common-menu-file-menu"))
        .entry(platform_menus::mac::file::new_file())
        .entry(
            MenuItem::new(
                LocalizedString::new("common-menu-file-open"),
            )
                .command(commands::SHOW_OPEN_PANEL.with(FileDialogOptions::new().multi_selection()))
                .hotkey(SysMods::Cmd, "o"),
        )
        .separator()
        .entry(platform_menus::mac::file::close())
}

fn button() -> impl Widget<AppState> {
    Flex::row()
        .with_child(Button::new("Open").on_click(|ctx, _data: &mut AppState, _env| {
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
            Button::new("Convert").on_click(|ctx, data: &mut AppState, _env| {
                ctx.submit_command(PROCESSING);
                for file in data.files.clone() {
                    resize_image(file.clone(), &data.watermark);
                    ctx.submit_command(MESSAGE.with(format!("done: {:?}", file.clone())));
                    &data.remove_file(file);
                }
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

fn resize_image(file: String, watermark: &str) {
    let path = Path::new(&file);
    let origin_image = image::open(&file).unwrap();
    log::info!("origin size: {}x{}", origin_image.width(), origin_image.height());

    let mut scale;
    if origin_image.width() < IMAGE_MAX_WIDTH || origin_image.height() < IMAGE_MAX_WIDTH {
        scale = origin_image;
        log::info!("image width or height < {:?}", IMAGE_MAX_WIDTH);
    } else {
        let resize_height: f32 = origin_image.height() as f32 / origin_image.width() as f32 * 3072.0;
        let resize_width = IMAGE_MAX_WIDTH;
        log::info!("resize size: {}x{}", resize_width, resize_height);
        scale = origin_image.resize(resize_width, resize_height as u32, FilterType::Nearest);
    }

    let new_file_name = thumb_output_path(path);
    let mut output = File::create(&new_file_name).unwrap();

    if watermark != "" {
        let wm_image = image::open(&Path::new(watermark)).ok().expect("Opening image failed");
        image::imageops::overlay(&mut scale, &wm_image, 100, 20);
        scale.write_to(&mut output, ImageFormat::Jpeg).unwrap();
    } else {
        scale.write_to(&mut output, ImageFormat::Jpeg).unwrap();
    }
}

fn thumb_output_path(path: &Path) -> String {
    let parent = path.parent().unwrap();
    let file_name = path.file_name().unwrap();
    let prefix = parent.join(file_name);
    let new_file_name = format!("{}-thumb.jpg", prefix.display());
    new_file_name
}

pub fn main() {
    let title = "Hug8217";

    let main_window = WindowDesc::new(make_ui())
        .window_size((512., 384.))
        .with_min_size((512., 384.))
        .menu(make_menu)
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
