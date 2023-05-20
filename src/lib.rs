use jpeg_decoder::Error as JpegError;
use png::ColorType;
use std::fs::File;
use std::io::{BufWriter, Read};
use std::sync::Arc;
use thiserror::Error;

// TODO use alpha as "replacement"
/// Colour distance threshold to consider what is part of the foreground (during first pass).
const LOW_PASS_THRESHOLD: u8 = 60;
/// Colour distance threshold to consider what is part of the background (during second pass).
const HIGH_PASS_THRESHOLD_2: u8 = 80;

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("error decoding jpeg: {0}")]
    Decode(JpegError),
    #[error("the input image does not have any metadata. resolution cannot be read.")]
    NoMetadata,
    #[error("invalid pixel count. is the image in the RGB pixel format?")]
    InvalidPixelCount,
}

/// Represents a coloured (RGB) pixel.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

pub fn jpeg_to_svg<T>(reader: T) -> Result<Vec<u8>, ConversionError>
where
    T: Read,
{
    let mut decoder = jpeg_decoder::Decoder::new(reader);
    let raw_pixels = decoder.decode().map_err(|e| ConversionError::Decode(e))?;
    println!("pixels length: {}", raw_pixels.len());
    println!("metadata: {:#?}", decoder.info());
    let Some(metadata) = decoder.info() else {
        return Err(ConversionError::NoMetadata);
    };
    let pixels = parse_pixels(raw_pixels)?;
    // First pass.
    let average_bg_pixel = compute_average_pixel(&pixels);
    println!("average bg pixel: {:#?}", average_bg_pixel);
    let mut low_pass_pixels = pixels.clone();
    filter_pixels_by_threshold(
        &mut low_pass_pixels,
        &average_bg_pixel,
        &Pixel::BLACK,
        LOW_PASS_THRESHOLD,
    );
    // Second pass.
    let average_fg_pixel = compute_average_pixel_ignoring(&low_pass_pixels, &Pixel::BLACK);
    println!("average fg pixel: {:#?}", average_fg_pixel);
    let mut high_pass_pixels = pixels.clone();
    filter_pixels_by_threshold(
        &mut high_pass_pixels,
        &average_fg_pixel,
        &Pixel::BLACK,
        HIGH_PASS_THRESHOLD_2,
    );
    // All high-pass pixels are removed from the low-pass pixels.
    let mut pixels_result = low_pass_pixels
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let is_high_pass = high_pass_pixels[i] != Pixel::BLACK;
            if is_high_pass {
                Pixel::BLACK
            } else {
                p.clone()
            }
        })
        .collect::<Vec<_>>();
    repeat_filter_by_neighbour_count(
        &mut pixels_result,
        4,
        metadata.width,
        metadata.height,
        &Pixel::BLACK,
        50,
    );
    save_debug_png(
        "debug-low-pass.png",
        &low_pass_pixels,
        metadata.width as u32,
        metadata.height as u32,
    );
    save_debug_png(
        "debug-high-pass.png",
        &high_pass_pixels,
        metadata.width as u32,
        metadata.height as u32,
    );
    save_debug_png(
        "debug-final.png",
        &pixels_result,
        metadata.width as u32,
        metadata.height as u32,
    );
    Ok(vec![])
}

fn repeat_filter_by_neighbour_count(
    pixels: &mut Vec<Pixel>,
    threshold: u8,
    width: u16,
    height: u16,
    replacement: &Pixel,
    max_count: usize,
) {
    let mut last_filtered_count;
    for _ in 0..max_count {
        last_filtered_count =
            filter_by_neighbour_count(pixels, threshold, width, height, replacement);
        if last_filtered_count == 0 {
            // No need to keep iterating.
            return;
        }
    }
}

fn filter_by_neighbour_count(
    pixels: &mut Vec<Pixel>,
    threshold: u8,
    width: u16,
    height: u16,
    replacement: &Pixel,
) -> u32 {
    let mut filtered_count: u32 = 0;
    let pixels_read = pixels.clone();
    for (i, p) in pixels.iter_mut().enumerate() {
        if p == replacement {
            continue;
        }
        let neighbour_indices = get_pixel_neighbour_indices(i, width, height);
        let actual_neighbour_count = neighbour_indices.iter().fold(0, |count, i| {
            if pixels_read[*i] == *replacement {
                count
            } else {
                count + 1
            }
        });
        if actual_neighbour_count < (threshold as usize) {
            *p = replacement.clone();
            filtered_count += 1;
        }
    }
    println!("filtered by neighbour count: {}", filtered_count);
    filtered_count
}

/// Returns up to 8 indices.
fn get_pixel_neighbour_indices(pixel_i: usize, width: u16, height: u16) -> Vec<usize> {
    let x = (pixel_i % width as usize) as i16;
    let y = (pixel_i / width as usize) as i16;
    let mut neighbours = Vec::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            // Skip the pixel itself.
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x + dx;
            let ny = y + dy;
            // Check the neighbour coordinates are valid.
            if nx >= 0 && nx < width as i16 && ny >= 0 && ny < height as i16 {
                // Convert the neighbour coordinates back into a 1D index.
                let neighbour_i = (ny as usize * width as usize) + nx as usize;
                neighbours.push(neighbour_i);
            }
        }
    }
    neighbours
}

fn filter_pixels_by_threshold(
    pixels: &mut Vec<Pixel>,
    reference: &Pixel,
    replacement: &Pixel,
    threshold: u8,
) {
    let filtered_count = pixels.iter_mut().fold(0, |c, p| {
        if p.exceeds_colour_threshold(&reference, threshold) {
            c
        } else {
            *p = replacement.clone();
            c + 1
        }
    });
    let filtered_percent = ((filtered_count as f32) * 100.0 / (pixels.len() as f32)) as u32;
    println!(
        "filtered pixel count: {} ({}%)",
        filtered_count, filtered_percent
    );
}

fn save_debug_png(path: &str, pixels: &Vec<Pixel>, width: u32, height: u32) {
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(ColorType::Rgb);
    let mut writer = encoder.write_header().unwrap();
    let data = pixels
        .iter()
        .flat_map(|p| [p.r, p.g, p.b])
        .collect::<Vec<_>>();
    writer.write_image_data(&data).unwrap();
}

fn parse_pixels(raw_pixels: Vec<u8>) -> Result<Vec<Pixel>, ConversionError> {
    let mut pixels = Vec::new();
    let count = raw_pixels.len();
    if count % 3 != 0 {
        return Err(ConversionError::InvalidPixelCount);
    }
    for pixel_n in 0..(count / 3) {
        let i = pixel_n * 3;
        pixels.push(Pixel {
            r: raw_pixels[i],
            g: raw_pixels[i + 1],
            b: raw_pixels[i + 2],
        })
    }
    Ok(pixels)
}

fn compute_average_pixel(pixels: &Vec<Pixel>) -> Pixel {
    let count = pixels.len() as u128;
    let avg_r = (pixels.iter().fold(0, |a, b| a + (b.r as u128)) / count) as u8;
    let avg_g = (pixels.iter().fold(0, |a, b| a + (b.g as u128)) / count) as u8;
    let avg_b = (pixels.iter().fold(0, |a, b| a + (b.b as u128)) / count) as u8;
    Pixel {
        r: avg_r,
        g: avg_g,
        b: avg_b,
    }
}

fn compute_average_pixel_ignoring(pixels: &Vec<Pixel>, ignore: &Pixel) -> Pixel {
    let filtered = pixels
        .iter()
        .filter(|p| *p != ignore)
        .cloned()
        .collect::<Vec<_>>();
    compute_average_pixel(&filtered)
}

impl Pixel {
    pub const BLACK: Pixel = Pixel { r: 0, g: 0, b: 0 };

    /// Checks whether the pixel is different than the reference by a threshold.
    pub fn exceeds_colour_threshold(&self, reference: &Pixel, threshold: u8) -> bool {
        let diff_magnitude = self.diff(reference).magnitude();
        diff_magnitude >= threshold
    }

    /// Computes the vector magnitude of the pixel in the RGB space.
    pub fn magnitude(&self) -> u8 {
        let to_sq = |v: u8| (v as f32).powi(2);
        let r_sq = to_sq(self.r);
        let g_sq = to_sq(self.g);
        let b_sq = to_sq(self.b);
        f32::sqrt(r_sq + g_sq + b_sq) as u8
    }

    /// Generates a diff pixel by comparing each field.
    pub fn diff(&self, other: &Pixel) -> Pixel {
        let calc_diff = |a: u8, b: u8| if a >= b { a - b } else { b - a };
        Pixel {
            r: calc_diff(self.r, other.r),
            g: calc_diff(self.g, other.g),
            b: calc_diff(self.b, other.b),
        }
    }
}
