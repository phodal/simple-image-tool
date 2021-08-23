use std::sync::Arc;

use druid::{AppDelegate, Command, DelegateCtx, Env, Handled, Selector, Target};

use crate::app_state::AppState;

pub const WATERMARK: Selector = Selector::new("simple.watermark");
pub const PROCESSING: Selector = Selector::new("simple.processing");
pub const OPENING: Selector = Selector::new("simple.opening");
pub const DONE: Selector = Selector::new("simple.done");
pub const MESSAGE: Selector<String> = Selector::new("simple.message");
pub const CONVERT: Selector = Selector::new("simple.convert");

#[derive(Debug, Default)]
pub struct SitDelegate;

impl AppDelegate<AppState> for SitDelegate {
    fn command<'a>(&mut self, ctx: &mut DelegateCtx<'a>, _target: Target, cmd: &Command, data: &mut AppState, _env: &Env) -> Handled {
        if let Some(info) = cmd.get(druid::commands::OPEN_FILE) {
            let file_arc = Arc::from(info.path().to_owned());

            match data.process_type.as_str() {
                "watermark" => {
                    ctx.submit_command(MESSAGE.with(format!("watermark: {:?}", info.path().display())));
                    data.set_watermark(file_arc);
                }
                _ => {
                    data.add_file(file_arc);
                }
            }

            return Handled::Yes;
        }
        if let Some(_) = cmd.get(CONVERT) {
            return Handled::Yes;
        }
        if let Some(_) = cmd.get(OPENING) {
            data.set_process_type("");
            return Handled::Yes;
        }
        if let Some(_) = cmd.get(WATERMARK) {
            data.set_process_type("watermark");
            return Handled::Yes;
        }
        if let Some(_) = cmd.get(DONE) {
            return Handled::Yes;
        }
        if let Some(msg) = cmd.get(MESSAGE) {
            data.add_message(msg.clone());
            return Handled::Yes;
        }

        return Handled::No;
    }
}
