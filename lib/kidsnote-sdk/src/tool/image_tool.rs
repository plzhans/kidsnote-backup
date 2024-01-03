use std::fs;

use image::{RgbImage, Rgb};
use rusttype::{Scale, Font};

pub struct ImageTool { 

}

impl ImageTool {

    pub fn text_to_image(title:&str, contents:&Vec<&str>, imag_path:&str) {

        let mut max_len = title.len();
        for text in contents {
            if max_len < text.len() {
                max_len = text.len();
            }
        }

        
        let width = (max_len as u32)* 8;
        let height: u32 = (contents.len() as u32)* 42 + 20;
      
        let mut img = RgbImage::new(width, height);

          // 배경을 흰색으로 채우기
        for pixel in img.pixels_mut() {
            *pixel = Rgb([255, 255, 255]);
        }

          // 텍스트 추가
        let font_bytes = include_bytes!("../../fonts/NanumSquareL.ttf"); // 여기에 사용할 폰트 파일 경로를 지정하세요
        let font = Font::try_from_bytes(font_bytes as &[u8]).expect("Unable to load font");


        let scale = Scale { x: 24.0, y: 24.0 }; // 폰트 크기 조정
        imageproc::drawing::draw_text_mut(
            &mut img,
            Rgb([0, 0, 0]),
            10,
            10,
            scale,
            &font,
            title,
        );
        
        let mut y_position = 60.0;
        for text in contents {
            imageproc::drawing::draw_text_mut(
                &mut img,
                Rgb([0, 0, 0]),
                10,
                y_position as i32,
                scale,
                &font,
                text,
            );
            y_position += scale.y * 1.5; // 행 간격 설정
        }

        if let Some(parent_dir) = std::path::Path::new(imag_path).parent() {
            if !parent_dir.exists() {
                match fs::create_dir_all(parent_dir) {
                    Ok(()) => {}
                    Err(_err) => {}
                }
            }
        }
        
        img.save(imag_path).expect("Unable to save image");

    }
} 