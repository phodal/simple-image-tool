use druid::{commands, Env, FileDialogOptions, LocalizedString, Menu, MenuItem, SysMods, WindowId};
use druid::Data;
use druid::menu::sys;

use crate::app_state::AppState;
use crate::sit_delegate::{CONVERT, WATERMARK};

pub fn make_menu(_: Option<WindowId>, _state: &AppState, _: &Env) -> Menu<AppState> {
    let mut menu = Menu::empty();
    #[cfg(target_os = "macos")]
        {
            menu = menu.entry(sys::mac::application::default());
        }

    menu.entry(file_menu())
}

fn file_menu<T: Data>() -> Menu<T> {
    Menu::new(LocalizedString::new("common-menu-file-menu"))
        .entry(sys::mac::file::new_file())
        .entry(
            MenuItem::new(
                LocalizedString::new("common-menu-file-open"),
            )
                .command(commands::SHOW_OPEN_PANEL.with(FileDialogOptions::new().multi_selection()))
                .hotkey(SysMods::Cmd, "o"),
        )
        .entry(
            MenuItem::new(
                LocalizedString::new("convert")
            )
                .command(CONVERT)
                .hotkey(SysMods::Cmd, "r"),
        )
        .entry(
            MenuItem::new(
                LocalizedString::new("Watermark")
            )
                .command(WATERMARK)
                .hotkey(SysMods::Cmd, "w"),
        )
        .separator()
        .entry(sys::mac::file::close())
}
