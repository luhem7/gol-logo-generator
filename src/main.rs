use std::fs;
use image::{imageops::FilterType, DynamicImage, ImageBuffer, Rgb};
use palette::{FromColor, Lch, Srgb, ShiftHueAssign,};

mod geom_helpers;
use crate::geom_helpers::{calc_eucledian_distance, Point, Viewport};


struct ImgSize (u32, u32);


const WHITE: [u8; 3] = [255, 255, 255];


fn draw_circle(circle_center: Point, circle_radius: f32, circle_color: Lch, imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Poorly optimized code. I should be operating on arrays of u32's instead of using formal Point structs.
        let curr_point = Point::new_u32(x, y);

        let distance_to_center = calc_eucledian_distance(&circle_center, &curr_point);

        if distance_to_center <= circle_radius {
            *pixel = image::Rgb(Srgb::from_color(circle_color).into_format().into());
        }
    }
}


fn main() {
    let logo_sizes: [ImgSize; 2] = [
        ImgSize(100, 100),
        ImgSize(200, 200),
    ];
    let source_size = ImgSize(800, 800);

    let viewport = Viewport::new(source_size.0, source_size.1);
    let circle_radius = source_size.0 as f32 / 8.0;

    let mut imgbuf = image::ImageBuffer::new(source_size.0, source_size.1);
    for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb(WHITE);
    }

    let circle_centers = [
        (0.0, 0.0),
        (0.0, source_size.1 as f32 / 3.0),
        (source_size.0 as f32 / 3.0, 0.0),
        (source_size.0 as f32 / 3.0, -1.0*source_size.1 as f32 / 3.0),
        (-1.0*source_size.0 as f32 / 3.0, -1.0*source_size.1 as f32 / 3.0)
    ];

    let mut circle_color: Lch = Lch::new(39.0, 132.0, 299.0);
    let hue_delta = 360.0 / circle_centers.len() as f32;
    for circle_center in circle_centers {
        let circle_center_p = viewport.translate(Point::new(circle_center.0, circle_center.1));
        draw_circle(circle_center_p, circle_radius, circle_color, &mut imgbuf);
        circle_color.shift_hue_assign(hue_delta)
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
