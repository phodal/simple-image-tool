extern crate image;

use std::fs::File;
use std::path::Path;
use std::sync::Arc;

use druid::{AppDelegate, AppLauncher, Color, Command, DelegateCtx, Env, Handled, Menu, Selector, Target, Widget, WidgetExt, WindowDesc, WindowId};
use druid::{
    commands, Data, FileDialogOptions, LocalizedString, MenuItem, platform_menus, SysMods,
};
use druid::widget::{Button, FillStrat, Flex};
use image::{GenericImageView, ImageFormat};
use image::imageops::FilterType;

use app_state::AppState;
use components::gallery::Gallery;
use components::message_box::MessageBox;

const LIGHTER_GREY: Color = Color::rgb8(242, 242, 242);

pub mod app_state;
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

pub const WATERMARK: Selector = Selector::new("simple.watermark");
pub const PROCESSING: Selector = Selector::new("simple.processing");
pub const OPENING: Selector = Selector::new("simple.opening");
pub const DONE: Selector = Selector::new("simple.done");
pub const MESSAGE: Selector<String> = Selector::new("simple.message");

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

fn resize_image(file: String, watermark: &str) {
    let path = Path::new(&file);
    let origin_image = image::open(&file).unwrap();
    log::info!("origin size: {}x{}", origin_image.width(), origin_image.height());

    let mut scale;
    if origin_image.width() < 3072 || origin_image.height() <  3072 {
        scale = origin_image;
        log::info!("image width or height < 3072");
    } else {
        let resize_height: f32 = origin_image.height() as f32 / origin_image.width() as f32 * 3072.0;
        let resize_width = 3072;
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

#[derive(Debug, Default)]
pub struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command<'a>(&mut self, ctx: &mut DelegateCtx<'a>, _target: Target, cmd: &Command, data: &mut AppState, _env: &Env) -> Handled {
        if let Some(info) = cmd.get(druid::commands::OPEN_FILE) {
            if data.status == "watermark" {
                let path = info.path().clone();
                data.set_watermark(Arc::from(path.to_owned()));
                ctx.submit_command(MESSAGE.with(format!("watermark: {:?}", path.display())));
                return Handled::Yes;
            }
            data.add_file(Arc::from(info.path().to_owned()));
            return Handled::Yes;
        }
        if let Some(_) = cmd.get(PROCESSING) {
            data.set_status("processing");
            return Handled::Yes;
        }
        if let Some(_) = cmd.get(OPENING) {
            data.set_status("opening");
            return Handled::Yes;
        }
        if let Some(_) = cmd.get(WATERMARK) {
            data.set_status("watermark");
            return Handled::Yes;
        }
        if let Some(_) = cmd.get(DONE) {
            data.set_status("done");
            return Handled::Yes;
        }
        if let Some(msg) = cmd.get(MESSAGE) {
            data.add_message(msg.clone());
            return Handled::Yes;
        }

        return Handled::No;
    }
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
        .delegate(Delegate::default())
        .log_to_console()
        .launch(init_state)
        .expect("Failed to launch application");
}
