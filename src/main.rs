use std::error::Error;

use clap::Parser;
use image::{DynamicImage, GenericImageView, ImageError, imageops::FilterType::Nearest};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use terminal_size::{Height, Width, terminal_size};

const ASCII_CHARS: &[u8] = b"@%#*+=:. ";

#[derive(Parser)]
struct Args {
    // Path file gambar
    file: String,
    // command untuk lebar gambar yang baru pada output ascii
    #[arg(short, long, default_value_t = 100)]
    width: u32,
    #[arg(short, long, default_value_t = false)]
    color: bool,
}

fn get_terminal_width() -> u32 {
    if let Some((Width(w), Height(_h))) = terminal_size() {
        return w as u32;
    }
    100 // fallback jika tidak bisa baca terminal
}

fn ascii_grayscale(img: &DynamicImage) -> String {
    let gray = img.to_luma8();
    let (w, h) = gray.dimensions();

    // pemorsesan paralel dengan rayon
    let lines: Vec<String> = (0..h)
        .into_par_iter()
        .map(|y| {
            let mut line = String::new();
            for x in 0..w {
                let pixel = gray.get_pixel(x, y).0[0];
                line.push(pixel_to_ascii(pixel));
            }
            line
        })
        .collect();
    lines.join("\n")
}

fn ascii_color(img: &DynamicImage) -> String {
    let rgb = img.to_rgb8();
    let (w, h) = rgb.dimensions();

    let lines: Vec<String> = (0..h)
        .into_par_iter()
        .map(|y| {
            let mut line = String::new();
            for x in 0..w {
                let px = rgb.get_pixel(x, y);
                let (r, g, b) = (px[0], px[1], px[2]);

                let brightness = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8;
                let ch = pixel_to_ascii(brightness);

                let colored = format!("\x1b[38;2;{r};{g};{b}m{ch}\x1b[0m");
                line.push_str(&colored);
            }
            line
        })
        .collect();
    lines.join("\n")
}

fn load_and_resize(path: &str, new_width: u32) -> Result<DynamicImage, ImageError> {
    let img = image::open(path)?;

    // Menghitung rasio berdasarkan lebar yang baru
    let (width, height) = img.dimensions();
    let ratio = height as f32 / width as f32;

    // Karakter pada terminal lebih tinggi daripada lebar, jadi dikompensasi
    let new_height = (new_width as f32 * ratio * 0.55) as u32;

    Ok(img.resize_exact(new_width, new_height, Nearest))
}

fn pixel_to_ascii(value: u8) -> char {
    let index = (value as f32 / 255.0 * (ASCII_CHARS.len() - 1) as f32) as usize;
    ASCII_CHARS[index] as char
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let width = get_terminal_width() - 5;

    let img = load_and_resize(&args.file, width)?;

    let ascii = if args.color {
        ascii_color(&img)
    } else {
        ascii_grayscale(&img)
    };

    println!("{ascii}");

    Ok(())
}
