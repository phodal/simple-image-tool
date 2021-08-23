use std::fs::File;
use std::path::Path;
use std::sync::Arc;

use druid::{Data, EventCtx, Lens};
use druid::image::{GenericImageView, ImageFormat};
use druid::image::imageops::FilterType;
use druid::widget::FillStrat;

use crate::IMAGE_MAX_WIDTH;
use crate::sit_delegate::MESSAGE;

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

    pub fn process_files(&mut self, ctx: &mut EventCtx) {
        for file in self.files.clone() {
            resize_image(file.clone(), &self.watermark);
            ctx.submit_command(MESSAGE.with(format!("done: {:?}", file.clone())));
            &self.remove_file(file);
        }
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
