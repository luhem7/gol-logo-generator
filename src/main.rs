use std::fs;
use image::{imageops::FilterType, DynamicImage, ImageBuffer, Rgb};
use palette::{FromColor, Hsl, Srgb, ShiftHue, Desaturate};

mod geom_helpers;
use crate::geom_helpers::{calc_eucledian_distance, Point, Viewport, ImgSize};


const WHITE: [u8; 3] = [255, 255, 255];
const GRAY: [u8; 3] = [164, 164, 164];


fn draw_circle(circle_center: &Point, circle_radius: f32, pixel_color: [u8; 3], imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Poorly optimized code. I should be operating on arrays of u32's instead of using formal Point structs.
        let curr_point = Point::new_u32(x, y);

        let distance_to_center = calc_eucledian_distance(circle_center, &curr_point);
        if distance_to_center <= circle_radius {
            *pixel = image::Rgb(pixel_color);
        }
    }
}


fn draw_circle_rainbow(
    circle_center: &Point, 
    circle_radius: f32, 
    rainbow_center: &Point, 
    starting_color: Hsl, 
    imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Poorly optimized code. I should be operating on arrays of u32's instead of using formal Point structs.
        let curr_point = Point::new_u32(x, y);

        let distance_to_center = calc_eucledian_distance(circle_center, &curr_point);
        let distance_from_corner = calc_eucledian_distance(&curr_point, rainbow_center) / 4.0;
        if distance_to_center <= circle_radius {
            *pixel = image::Rgb(Srgb::from_color(starting_color.shift_hue(distance_from_corner)).into_format().into());
        }
    }
}


fn draw_circle_rainbow_bw(
    circle_center: &Point, 
    circle_radius: f32, 
    rainbow_center: &Point, 
    starting_color: Hsl,
    imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Poorly optimized code. I should be operating on arrays of u32's instead of using formal Point structs.
        let curr_point = Point::new_u32(x, y);

        let distance_to_center = calc_eucledian_distance(circle_center, &curr_point);
        let distance_from_corner = calc_eucledian_distance(&curr_point, rainbow_center) / 4.0;
        if distance_to_center <= circle_radius {
            let desat_color = starting_color.shift_hue(distance_from_corner).desaturate(0.8);
            *pixel = image::Rgb(Srgb::from_color(desat_color).into_format().into());
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

    let viewport: Viewport = Viewport::from(&source_size);

    let default_pixel = image::Rgb(WHITE);
    let mut imgbuf = image::ImageBuffer::from_pixel(source_size.0, source_size.1, default_pixel);
    
    let circle_radius = source_size.0 as f32 / 10.0;
    let dist_ratio = 4.5;

    let adjusted_dot_center = ((-1.0*source_size.0 as f32 / dist_ratio), 0.0);
    let dot_center_translated = viewport.translate(adjusted_dot_center.into());

    let top_right_center = (source_size.0 as f32 / dist_ratio, source_size.1 as f32 / dist_ratio);
    let circle_centers = [
        (0.0, source_size.1 as f32 / dist_ratio),
        top_right_center,
        adjusted_dot_center,
        (source_size.0 as f32 / dist_ratio, 0.0),
        (source_size.0 as f32 / dist_ratio, -1.0*source_size.1 as f32 / dist_ratio),
    ];
    
    let starting_color: Hsl = Hsl::new(80.0, 0.90, 0.5);
    let starting_color_bw: Hsl = Hsl::new(80.0, 0.90, 0.1);
    let rainbow_center: Point = viewport.translate(Point::from(top_right_center));

    draw_circle_rainbow_bw(
        &dot_center_translated,
        source_size.0 as f32 *2.0,
        &rainbow_center,
        starting_color_bw,
        &mut imgbuf
    );

    for circle_center in circle_centers {
        let circle_center_p = viewport.translate(Point::new(circle_center.0, circle_center.1));

        draw_circle(&circle_center_p, circle_radius+3.0, GRAY, &mut imgbuf);

        draw_circle_rainbow(
            &circle_center_p, 
            circle_radius, 
            &rainbow_center, 
            starting_color, 
            &mut imgbuf);
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
