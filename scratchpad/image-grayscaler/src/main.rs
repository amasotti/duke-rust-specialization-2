//! # Image Grayscaler (fruits)
//!
//! `image-grayscaler` is a library for converting images to grayscale.
//!
//! This is the main binary for the `image-grayscaler` library.
//!
//! ## Usage
//!
//! ```sh
//! image-grayscaler --input <input> --output <output>
//! # or
//! image-grayscaler -i <input> -o <output>
//! ```
//!
//! - `input`: The path to the input image
//! - `output`: The path to the output image
//!
//! ## Example
//!
//! ```sh
//! image-grayscaler -i input.jpg -o output.jpg
//! ```
//!
//! This will convert the `input.jpg` image to grayscale and save it as `output.jpg`.
//!
//! ## Notes
//!
//! Supported image formats: jpg, jpeg, png, gif, bmp, ico, tiff, webp
//!
//! ## Author
//!
//! Toni Masotti

use clap::Parser;
use image_grayscaler::ImageGrayscaler;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Toni Masotti",
    about = "Converts an image to grayscale"
)]
struct Options {
    #[clap(short, long)]
    input: String,
    #[clap(short, long)]
    output: String,
}

fn main() {
    env_logger::init();
    let options = Options::parse();
    println!("Input: {}", options.input);
    println!("Output: {}", options.output);

    let mut grayscaler = ImageGrayscaler::new(&options.input);
    grayscaler.convert_to_grayscale(&options.output);
}
