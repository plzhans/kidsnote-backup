use std::fs;

use filetime::FileTime;
use image::{Rgb, RgbImage};
use rusttype::{Font, Scale};

use crate::auth::error_types::AuthError;

pub struct ImageTool {}

impl ImageTool {
    pub fn text_to_image(
        title: &str,
        author_name: &str,
        contents: &Vec<&str>,
        imag_path: &str,
        file_time: FileTime,
    ) -> Result<(), AuthError> {
        let mut max_len = title.len();
        for text in contents {
            if max_len < text.len() {
                max_len = text.len();
            }
        }

        let width = (max_len as u32) * 8 + 20;
        let height: u32 = ((contents.len() + 1) as u32) * 42 + 50;

        let mut img = RgbImage::new(width, height);

        // 배경을 흰색으로 채우기
        for pixel in img.pixels_mut() {
            *pixel = Rgb([255, 255, 255]);
        }

        // 텍스트 추가
        let font_bytes = include_bytes!("../../fonts/NanumSquareL.ttf"); // 여기에 사용할 폰트 파일 경로를 지정하세요
        let font = Font::try_from_bytes(font_bytes as &[u8]).expect("Unable to load font");
        let scale = Scale { x: 24.0, y: 24.0 }; // 폰트 크기 조정

        // 제목
        imageproc::drawing::draw_text_mut(&mut img, Rgb([0, 0, 0]), 10, 10, scale, &font, title);

        // 작성자
        imageproc::drawing::draw_text_mut(
            &mut img,
            Rgb([0, 0, 0]),
            10,
            40,
            scale,
            &font,
            &format!("작성자 : {}", author_name),
        );

        // 내용
        let mut y_position = 90.0;
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

        match filetime::set_file_times(imag_path, file_time, file_time) {
            Ok(()) => {}
            Err(_err) => {}
        }

        Ok(())
    }
}
