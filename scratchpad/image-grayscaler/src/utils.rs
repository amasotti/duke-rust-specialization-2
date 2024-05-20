use image::ImageFormat;
use std::path::Path;

const RED_COEFF: f32 = 0.299;
const GREEN_COEFF: f32 = 0.587;
const BLUE_COEFF: f32 = 0.114;

/// Extracts the RGB values from a pixel
pub fn extrax_rgb(pixel: [u8; 4]) -> (u8, u8, u8) {
    (pixel[0], pixel[1], pixel[2])
}

/// Calculates the grayscale value of a pixel
/// Uses the ITU-R BT.709 formula
pub fn calc_gray(rbg: (u8, u8, u8)) -> u8 {
    let (r, g, b) = rbg;
    let gray_f32 = RED_COEFF * r as f32 + GREEN_COEFF * g as f32 + BLUE_COEFF * b as f32;
    gray_f32 as u8
}

/// Determines the image format based on the file extension
/// Supported formats: jpg, jpeg, png, gif, bmp, ico, tiff, webp
///
/// @internal
///
/// # Panics
/// Panics if the extension is not supported
///
pub fn determine_format(path: &Path) -> ImageFormat {
    let extension = path
        .extension()
        .expect("Could not get extension")
        .to_str()
        .expect("Could not convert extension to string");

    match extension {
        "jpg" | "jpeg" => image::ImageFormat::Jpeg,
        "png" => image::ImageFormat::Png,
        "gif" => image::ImageFormat::Gif,
        "bmp" => image::ImageFormat::Bmp,
        "ico" => image::ImageFormat::Ico,
        "tiff" => image::ImageFormat::Tiff,
        "webp" => image::ImageFormat::WebP,
        _ => panic!("Unsupported image format"),
    }
}
