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
            if data.process_type == "watermark" {
                let path = info.path().clone();
                data.set_watermark(Arc::from(path.to_owned()));
                ctx.submit_command(MESSAGE.with(format!("watermark: {:?}", path.display())));
                return Handled::Yes;
            }

            data.add_file(Arc::from(info.path().to_owned()));
            return Handled::Yes;
        }
        if let Some(_) = cmd.get(CONVERT) {
            return Handled::Yes;
        }
        if let Some(_) = cmd.get(OPENING) {
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
