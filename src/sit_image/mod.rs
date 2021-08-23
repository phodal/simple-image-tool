use std::fs::File;
use std::path::Path;

use druid::image::{GenericImageView, ImageFormat};
use druid::image::imageops::FilterType;

use crate::IMAGE_MAX_WIDTH;

pub fn resize_image(file: String, watermark: &str) {
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
