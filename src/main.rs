use druid::{WindowDesc, AppLauncher, Widget, WidgetExt, Color, AppDelegate, DelegateCtx, Target, Command, Env, Handled, Menu, WindowId};
use druid::widget::{Flex, FillStrat};

const LIGHTER_GREY: Color = Color::rgb8(242, 242, 242);

use druid::{
    commands, platform_menus, Data, FileDialogOptions, LocalizedString, MenuItem, SysMods, Lens,
};
use std::path::Path;
use std::sync::Arc;
use crate::gallery::Gallery;

pub mod gallery;

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

#[derive(Clone, Lens)]
pub struct AppState {
    pub fill_strat: FillStrat,
    pub title: String,
    pub files: Vec<String>,
}

impl AppState {
    pub fn add_file(&mut self, path: Arc<Path>) {
        match path.extension() {
            None => {}
            Some(result) => {
                let ext = format!("{}", result.to_str().unwrap());
                if ext == "jpg" || ext == "png" {
                    log::info!("add file: {:?}", path.display());
                    self.files.push(format!("{:?}", path.display()));
                }
            }
        }
    }
}

impl Data for AppState {
    fn same(&self, other: &Self) -> bool {
        self.title.same(&other.title)
            && self.files.len() == other.files.len()
            && self
            .files
            .iter()
            .zip(other.files.iter())
            .all(|(a, b)| a.same(b))
    }
}

fn make_ui() -> impl Widget<AppState> {
    let flex = Flex::column();
    flex.with_child(Gallery::new())
        // .with_child(
        //     Button::new("Convert").on_click(|ctx, data: &mut AppState, _env| {
        //         // todo
        //     })
        // )
        .background(LIGHTER_GREY)
}

#[derive(Debug, Default)]
pub struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command<'a>(&mut self, ctx: &mut DelegateCtx<'a>, _target: Target, cmd: &Command, data: &mut AppState, _env: &Env) -> Handled {
        if let Some(info) = cmd.get(druid::commands::OPEN_FILE) {
            data.add_file(Arc::from(info.path().to_owned()));
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
    };

    AppLauncher::with_window(main_window)
        .delegate(Delegate::default())
        .log_to_console()
        .launch(init_state)
        .expect("Failed to launch application");
}
