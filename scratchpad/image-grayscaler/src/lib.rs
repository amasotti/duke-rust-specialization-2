//! # Image Grayscaler
//!
//! `image-grayscaler` is a library for converting images to grayscale.
//!
//! ## Design
//!
//! The `ImageGrayscaler` struct is the main entry point for the library.
//! It provides methods for loading an image, converting it to grayscale, and saving the result.
//!
//! The `ImageGrayscaler` struct uses the [image](https://docs.rs/image/latest/image/) crate for image processing.
//!
//! ## Example
//!
//! ```rust
//! use image_grayscaler::ImageGrayscaler;
//!
//! let input = "input.jpg";
//! let output = "output.jpg";
//!
//! let mut grayscaler = ImageGrayscaler::new(input);
//!
//! // Convert the image to grayscale
//! grayscaler.convert_to_grayscale(output);
//!
//! println!("Image converted successfully");
//! ```
//!
//! ## Supported Formats
//!
//! The library supports the following image formats:
//! - jpg
//! - jpeg
//! - png
//! - gif
//! - bmp
//! - ico
//! - tiff
//! - webp
//!

mod utils;

use image::{DynamicImage, GenericImageView, ImageBuffer};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// The `ImageGrayscaler` struct provides methods for loading an image, converting it to grayscale, and saving the result.
#[allow(dead_code)]
pub struct ImageGrayscaler {
    /// The path to the input image
    input: String,

    /// The loaded image
    image: DynamicImage,
}

/// Implementation of `ImageGrayscaler`
///
/// It contains methods for loading an image, converting it to grayscale, and saving the result.
impl ImageGrayscaler {
    /// Creates a new `ImageGrayscaler` instance with the given input image path.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice that holds the path to the input image file.
    ///
    /// # Returns
    ///
    /// A new `ImageGrayscaler` instance.
    ///
    /// # Panics
    ///
    /// Panics if the file cannot be opened or the image cannot be loaded.
    ///
    /// # Example
    ///
    /// ```rust
    /// use image_grayscaler::ImageGrayscaler;
    ///
    /// let input = "input.jpg";
    /// let grayscaler = ImageGrayscaler::new(input);
    /// ```
    pub fn new(input: &str) -> Self {
        let path = Path::new(input);
        let file = File::open(path).unwrap_or_else(|_| panic!("Could not open file: {}", input));
        let reader = BufReader::new(file);

        let format = utils::determine_format(path);

        let image = image::load(reader, format).expect("Could not load image");

        println!("Image loaded successfully");

        ImageGrayscaler {
            input: input.to_string(),
            image,
        }
    }

    /// Converts the loaded image to grayscale and saves it to the specified output path.
    /// It utilizes the luma8 color type for the grayscale conversion directly from the
    /// image crate.
    ///
    /// # Arguments
    ///
    /// * `output_path` - A string slice that holds the path to the output image file.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use image_grayscaler::ImageGrayscaler;
    ///
    /// let input = "input.jpg";
    /// let output = "output.jpg";
    ///
    /// let mut grayscaler = ImageGrayscaler::new(input);
    ///
    /// // Convert the image to grayscale
    /// grayscaler.luam_convert_to_grayscale(output);
    ///
    /// println!("Image converted successfully");
    /// ```
    pub fn luma_convert_to_grayscale(&mut self, output_path: &str) {
        let luma = self.image.to_luma8();
        luma.save(output_path).expect("Could not save image");
        println!("Image saved successfully");
    }

    /// Converts the loaded image to grayscale and saves it to the specified output path.
    /// It utilizes the image buffer to create a new image with the grayscale pixels.
    /// This is a more low-level approach compared to the `to_luma8` method.
    ///
    /// # Arguments
    ///
    /// * `output_path` - A string slice that holds the path to the output image file.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use image_grayscaler::ImageGrayscaler;
    ///
    /// let input = "input.jpg";
    /// let output = "output.jpg";
    ///
    /// let mut grayscaler = ImageGrayscaler::new(input);
    ///
    /// // Convert the image to grayscale
    /// grayscaler.convert_to_grayscale(output);
    ///
    /// println!("Image converted successfully");
    ///
    /// ```
    ///
    /// # Note
    ///
    /// This method is more flexible and allows for custom grayscale conversion logic.
    pub fn convert_to_grayscale(&mut self, output_path: &str) {
        let transformed_image = self.image_to_black_and_white();
        let out_path = Path::new(output_path);
        transformed_image
            .save(out_path)
            .expect("Could not save image");

        println!("Image coverted and saved successfully");
    }

    fn image_to_black_and_white(&self) -> DynamicImage {
        let (w, h) = self.image.dimensions();
        let mut gray_image = ImageBuffer::new(w, h);

        for (x, y, pixel) in self.image.pixels() {
            let rgb = utils::extrax_rgb(pixel.0);
            let gray_val = utils::calc_gray(rgb);
            gray_image.put_pixel(x, y, image::Luma([gray_val]));
        }

        DynamicImage::ImageLuma8(gray_image)
    }
}
