use std::cmp::max;
use std::env;
use std::io::Cursor;
use std::process::Command;

use image::DynamicImage;
use image::imageops::FilterType;
use image::io::Reader as ImageReader;

fn get_code(byte: u8) -> u8 {
    match byte {
        b'A'..=b'Z' => byte - b'A',
        b'a'..=b'z' => byte - b'a' + 26,
        b'0'..=b'9' => byte - b'0' + 52,
        b'+' => 62,
        b'/' => 63,
        _ => 64, // 这里假设任何非法字符都返回64，可能需要根据实际编码规则调整
    }
}

fn decode(encoded: Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();
    let mut padding_count = 0;

    // 遍历字节向量，计算每个字节的6位二进制值
    for &byte in &encoded {
        if byte == b'=' {
            // 遇到填充字符，计算填充数量
            padding_count += 1;
            break;
        }
        let code = get_code(byte);
        result.extend(format!("{:06b}", code).as_bytes().iter().copied());
    }

    // 根据填充数量调整结果长度
    while padding_count > 0 {
        result.pop();
        padding_count -= 1;
    }

    // 将结果按8位一组进行分组，转换为字节
    let mut final_result = Vec::new();
    for chunk in result.chunks(8) {
        if chunk.len() == 8 {
            let num = u8::from_str_radix(std::str::from_utf8(chunk).unwrap(), 2).unwrap();
            final_result.push(num);
        }
    }
    final_result
}

fn main() {
    let mut path = "";
    let mut img: DynamicImage;
    let args: Vec<String> = env::args().collect();
    if let Some(first_args) = args.get(1) {
        path = first_args;
        if path.ends_with(".avif") {
            let output = Command::new(r".\lib\avif.exe")
                .arg(path)
                .output()
                .unwrap();
            let image_data = decode(output.stdout);
            img = ImageReader::new(Cursor::new(image_data)).with_guessed_format().unwrap().decode().unwrap();
        } else {
            img = match ImageReader::open(path) {
                Ok(img) => { img.decode().unwrap() }
                Err(_) => {
                    println!("Failed to read the image {}", path);
                    return;
                }
            };
        }
    } else {
        println!("Please specify a path to the image");
        return;
    }
    let width = img.width();
    let height = img.height();
    let resize_factor = 200f64 / max(width, height) as f64;
    if resize_factor < 1.0 {
        img = img.resize_exact((width as f64 * resize_factor) as u32,
                               (height as f64 * resize_factor * 0.45) as u32,
                               FilterType::Nearest);
    } else {
        img = img.resize_exact(width, (height as f64 * 0.45) as u32, FilterType::Nearest);
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
