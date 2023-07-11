mod geom_helpers;

use crate::geom_helpers::{calc_eucledian_distance, Point, Viewport};
use image::{imageops::FilterType, DynamicImage};


fn main() {
    let final_size = (100, 100);
    let imgx: u32 = 800;
    let imgy: u32 = 800;

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
            *pixel = image::Rgb([255, 255, 255]);
        } else {
            let r = (0.6 * x as f32) as u8;
            let b = (0.6 * y as f32) as u8;
            *pixel = image::Rgb([r, 0, b]);
        }
    }

    let scaled = DynamicImage::from(imgbuf).resize(100, 100, FilterType::Lanczos3);
    // imgbuf = image::imageops::blur(&imgbuf, 1.0);

    scaled.save("./outputs/logo.png").unwrap();
}
