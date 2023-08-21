use std::path::PathBuf;
use image::{io::Reader as ImageReader, Rgb};

pub fn sample_texture() -> Vec<Vec<(f32, f32, f32)>> {
    let mut texture = vec![];
    for _ in 0..100 {
        let mut row = vec![];
        for x in 0..100 {
            row.push(((x as f32) / 255.0, 0.0, 0.0));
        }
        texture.push(row);
    }
    texture
}

pub fn load_from(path: PathBuf) -> Vec<Vec<(f32, f32, f32)>> {
    let image = ImageReader::open(path).unwrap().decode().unwrap();
    let f32_image = image.to_rgb32f();
    let mut texture = vec![]; 
    f32_image.rows().for_each(|row| {
        let mut texture_row = vec![];
        row.for_each(|Rgb(pixel)| {
            texture_row.push((pixel[0], pixel[1], pixel[2]));
        });
        texture.push(texture_row);
    });
    texture
}


