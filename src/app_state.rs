use std::path::Path;
use std::sync::Arc;

use druid::{Data, Lens};
use druid::widget::FillStrat;

#[derive(Clone, Lens)]
pub struct AppState {
    pub fill_strat: FillStrat,
    pub title: String,
    pub files: Vec<String>,
    pub watermark: String,
    pub messages: Vec<String>,
    pub status: String,
}

impl AppState {
    pub fn add_file(&mut self, path: Arc<Path>) {
        log::info!("add file: {}", path.clone().display());
        match path.extension() {
            None => {}
            Some(result) => {
                let ext = format!("{}", result.to_str().unwrap()).to_lowercase();
                if ext == "jpg" || ext == "png" || ext == "jpeg" || ext == "webp" || ext == "bmp" {
                    log::info!("add file: {:?}", path.display());
                    self.files.push(format!("{}", path.display()));
                }
            }
        }
    }

    pub fn set_watermark(&mut self, path: Arc<Path>) {
        match path.extension() {
            None => {}
            Some(result) => {
                let ext = format!("{}", result.to_str().unwrap());
                if ext == "png" {
                    self.watermark = format!("{}", path.display());
                }
            }
        }
    }

    pub fn set_status(&mut self, status: &str) {
        self.status = status.to_string();
    }

    pub fn add_message(&mut self, msg: String) {
        self.messages.push(msg);
    }

    pub fn remove_file(&mut self, file: String) {
        let index = self.files.iter().position(|x| *x == file).unwrap();
        self.files.remove(index);
    }
}

impl Data for AppState {
    fn same(&self, other: &Self) -> bool {
        self.title.same(&other.title)
            && self.files.len() == other.files.len()
            // todo: add more message
            && self.messages.len() == other.messages.len()
            && self
            .files
            .iter()
            .zip(other.files.iter())
            .all(|(a, b)| a.same(b))
    }
}