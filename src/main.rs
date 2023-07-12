use std::fs;
use image::{imageops::FilterType, DynamicImage};

mod geom_helpers;
use crate::geom_helpers::{calc_eucledian_distance, Point, Viewport};


struct ImgSize (u32, u32);

fn main() {
    let logo_sizes: [ImgSize; 2] = [
        ImgSize(100, 100),
        ImgSize(200, 200),
    ];
    let source_size = ImgSize(800, 800);
    let imgx: u32 = source_size.0;
    let imgy: u32 = source_size.1;

    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    let viewport = Viewport::new(imgx, imgy);
    let circle_center = viewport.translate(Point::new(0.0, 0.0));
    let circle_radius = imgx as f32 / 4.0;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Poorly optimized code. I should be operating on arrays of u32's instead of using formal Point structs.
        let curr_point = Point::new_u32(x, y);

        let distance_to_center = calc_eucledian_distance(&circle_center, &curr_point);

        if distance_to_center <= circle_radius {
            let r = (0.6 * x as f32) as u8;
            let b = (0.6 * y as f32) as u8;
            *pixel = image::Rgb([r, 0, b]);
        } else {
            *pixel = image::Rgb([255, 255, 255]);
        }
    }

    let directory_path = "outputs";
    // Check if the directory exists
    if fs::metadata(directory_path).is_err() {
        // Directory does not exist, so create it
        fs::create_dir(directory_path).expect("Failed to create directory");
    }

    for logo_size in logo_sizes {
        let scaled = DynamicImage::from(imgbuf.clone()).resize(logo_size.0, logo_size.1, FilterType::Lanczos3);
        scaled.save(format!("./{}/logo_{}x{}.png", directory_path, logo_size.0, logo_size.1)).unwrap();
    }
}
