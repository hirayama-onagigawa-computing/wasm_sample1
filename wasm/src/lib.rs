use wasm_bindgen::prelude::*;
use image::*;

enum ConvertMode {
    Grayscale,
    FlipHorizontal,
    FlipVertical,
    RotateLeft,
    RotateRight,
    Blur,
}

fn convert(mode: ConvertMode, width: u32, height: u32, raw_data: Vec<u8>) -> Vec<u8> {
    let rgba = RgbaImage::from_raw(width, height, raw_data).expect("error");
    let di = DynamicImage::ImageRgba8(rgba);

    let converted = match mode {
        ConvertMode::Grayscale => {
            let gray = di.into_luma8();
            // 再度rgbaに戻す
            DynamicImage::ImageLuma8(gray)
        },
        ConvertMode::FlipHorizontal => di.fliph(),
        ConvertMode::FlipVertical => di.flipv(),
        ConvertMode::RotateLeft => di.rotate270(),
        ConvertMode::RotateRight => di.rotate90(),
        ConvertMode::Blur => di.blur(3.0),
    };

    converted.into_rgba8().to_vec()
}

#[wasm_bindgen]
pub fn grayscale(width: u32, height: u32, raw_data: Vec<u8>) -> Vec<u8> {
    convert(ConvertMode::Grayscale, width, height, raw_data)
}

#[wasm_bindgen]
pub fn flip_horizontal(width: u32, height: u32, raw_data: Vec<u8>) -> Vec<u8> {
    convert(ConvertMode::FlipHorizontal, width, height, raw_data)
}

#[wasm_bindgen]
pub fn flip_vertical(width: u32, height: u32, raw_data: Vec<u8>) -> Vec<u8> {
    convert(ConvertMode::FlipVertical, width, height, raw_data)
}

#[wasm_bindgen]
pub fn rotate_left(width: u32, height: u32, raw_data: Vec<u8>) -> Vec<u8> {
    convert(ConvertMode::RotateLeft, width, height, raw_data)
}

#[wasm_bindgen]
pub fn rotate_right(width: u32, height: u32, raw_data: Vec<u8>) -> Vec<u8> {
    convert(ConvertMode::RotateRight, width, height, raw_data)
}

#[wasm_bindgen]
pub fn blur(width: u32, height: u32, raw_data: Vec<u8>) -> Vec<u8> {
    convert(ConvertMode::Blur, width, height, raw_data)
}
