use std::fs;
use image::{imageops::FilterType, DynamicImage, ImageBuffer, Rgb};
use palette::{FromColor, Hsl, Srgb, ShiftHue};

mod geom_helpers;
use crate::geom_helpers::{calc_eucledian_distance, Point, Viewport};


struct ImgSize (u32, u32);
const WHITE: [u8; 3] = [255, 255, 255];


fn draw_circle(circle_center: Point, circle_radius: f32, pixel_color: [u8; 3], imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Poorly optimized code. I should be operating on arrays of u32's instead of using formal Point structs.
        let curr_point = Point::new_u32(x, y);

        let distance_to_center = calc_eucledian_distance(&circle_center, &curr_point);
        if distance_to_center <= circle_radius {
            *pixel = image::Rgb(pixel_color);
        }
    }
}


fn draw_circle_rainbow(circle_center: Point, circle_radius: f32, img_height: f32, starting_color: Hsl, imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Poorly optimized code. I should be operating on arrays of u32's instead of using formal Point structs.
        let curr_point = Point::new_u32(x, y);

        let distance_to_center = calc_eucledian_distance(&circle_center, &curr_point);
        let distance_from_corner = calc_eucledian_distance(&curr_point, &Point::new(0.0, img_height)) / 1.5;
        if distance_to_center <= circle_radius {
            *pixel = image::Rgb(Srgb::from_color(starting_color.shift_hue(distance_from_corner)).into_format().into());
        }
    }
}


fn main() {
    let logo_sizes: [ImgSize; 3] = [
        ImgSize(128, 128),
        ImgSize(256, 256),
        ImgSize(512, 512),
    ];
    let source_size = ImgSize(1024, 1024);

    let viewport = Viewport::new(source_size.0, source_size.1);
    let circle_radius = source_size.0 as f32 / 10.0;

    let mut imgbuf = image::ImageBuffer::new(source_size.0, source_size.1);
    draw_circle(viewport.translate(Point::new(0.0, 0.0)), source_size.0 as f32/2.0, WHITE, &mut imgbuf);
    // for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
    //     *pixel = image::Rgb(WHITE);
    // }

    let dist_ratio = 4.5;
    let circle_centers = [
        (0.0, source_size.1 as f32 / dist_ratio),
        (source_size.0 as f32 / dist_ratio, source_size.1 as f32 / dist_ratio),
        ((-1.0*source_size.0 as f32 / dist_ratio) + (source_size.0 as f32 / (dist_ratio*4.0)), 0.0),
        (source_size.0 as f32 / dist_ratio, 0.0),
        (source_size.0 as f32 / dist_ratio, -1.0*source_size.1 as f32 / dist_ratio),
    ];
    
    let pixel_color: Hsl = Hsl::new(0.0, 0.75, 0.5);

    for circle_center in circle_centers {
        let circle_center_p = viewport.translate(Point::new(circle_center.0, circle_center.1));
        draw_circle_rainbow(circle_center_p, circle_radius, source_size.1 as f32, pixel_color, &mut imgbuf);
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
