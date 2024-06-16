use std::cmp::max;
use std::env;
use image::imageops::FilterType;
use image::io::Reader as ImageReader;

fn main() {
    let mut path = "";
    let args: Vec<String> = env::args().collect();
    if let Some(first_args) = args.get(1) {
        path = first_args;
    }
    let mut img = match ImageReader::open(path) {
        Ok(img) => { img.decode().unwrap() }
        Err(_) => {
            println!("Failed to read the image {}", path);
            return;
        }
    };
    let width = img.width();
    let height = img.height();
    let resize_factor = 200f64 / max(width, height) as f64;
    if resize_factor < 1.0 {
        img = img.resize_exact((width as f64 * resize_factor) as u32,
                               (height as f64 * resize_factor * 0.45) as u32,
                               FilterType::Nearest);
    }
    let width = img.width();
    let height = img.height();
    let gray_img = img.to_luma8();
    let asciis = ['@', '%', '#', '*', '+', '=', ':', '.', ' '];
    let len_asciis = asciis.len();
    let mut result = String::from("");
    for i in 0..height {
        for j in 0..width {
            let gray_value = gray_img.get_pixel(j, i)[0];
            result.push(asciis[((gray_value as f64 / 255f64) * (len_asciis - 1) as f64) as usize])
        }
        result.push('\n');
    }
    println!("{}", result);
}
