use druid::{WindowDesc, AppLauncher, Widget, WidgetExt, Color, AppDelegate, DelegateCtx, Target, Command, Env, Handled};
use druid::widget::{Flex, Button};

const LIGHTER_GREY: Color = Color::rgb8(242, 242, 242);

use druid::{
    commands, platform_menus, Data, FileDialogOptions, LocalizedString, MenuDesc, MenuItem, SysMods, Lens
};

pub fn menus<T: Data>() -> MenuDesc<T> {
    let mut menu = MenuDesc::empty();
    #[cfg(target_os = "macos")]
        {
            menu = menu.append(platform_menus::mac::application::default());
        }

    menu.append(file_menu())
}

fn file_menu<T: Data>() -> MenuDesc<T> {
    MenuDesc::new(LocalizedString::new("common-menu-file-menu"))
        .append(platform_menus::mac::file::new_file().disabled())
        .append(
            MenuItem::new(
                LocalizedString::new("common-menu-file-open"),
                commands::SHOW_OPEN_PANEL.with(FileDialogOptions::new().multi_selection()),
            )
                .hotkey(SysMods::Cmd, "o"),
        )
        .append_separator()
        .append(platform_menus::mac::file::close())
}

#[derive(Clone, Lens)]
pub struct AppState {
    pub title: String,
    pub files: Vec<String>
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
    Flex::column()
        .with_child(
            Button::new("Convert").on_click(|ctx, data: &mut AppState, _env| {
                // todo
            })
        )
        .background(LIGHTER_GREY)
}

#[derive(Debug, Default)]
pub struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command<'a>(&mut self, ctx: &mut DelegateCtx<'a>, _target: Target, cmd: &Command, data: &mut AppState, _env: &Env) -> Handled {
        if let Some(info) = cmd.get(druid::commands::OPEN_FILE) {
            return Handled::Yes
        }

        return Handled::No
    }
}

pub fn main() {
    let title = "Hug8217";

    let main_window = WindowDesc::new(make_ui())
        .window_size((1024., 768.))
        .with_min_size((1024., 768.))
        .menu(menus())
        .title(title);

    let init_state = AppState {
        title: "".to_string(),
        files: vec![]
    };

    AppLauncher::with_window(main_window)
        .delegate(Delegate::default())
        .log_to_console()
        .launch(init_state)
        .expect("Failed to launch application");
}
