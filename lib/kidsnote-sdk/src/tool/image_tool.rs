use std::{fs::{self, File}, io::Write};

use chrono::{DateTime, Utc};
use filetime::FileTime;
use image::{Rgb, RgbImage};
use little_exif::{exif_tag::ExifTag, metadata::Metadata};
use rusttype::{Font, Scale};

use crate::auth::error_types::AuthError;

pub struct ImageTool {}

impl ImageTool {
    pub fn text_to_image_file(
        title: &str,
        center_name: &Option<String>,
        author_name: &str,
        contents: &Vec<&str>,
        file_path: &str,
        file_date: DateTime<Utc>,
    ) -> Result<(), AuthError> {
        let final_title = match center_name {
            Some(center_name) 
                if center_name != author_name 
                && author_name.find(center_name).is_none() 
            => {
                format!("{} ({})\n", title, center_name)
            }
            _ => {
                format!("{}\n", title)
            }
        };
        let mut max_len = final_title.len();
        for text in contents {
            if let Some(offset) =  text.find('\n') {
                if max_len < offset {
                    max_len = offset;
                }
            } else {
                if max_len < text.len() {
                    max_len = text.len();
                }
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
        imageproc::drawing::draw_text_mut(&mut img, Rgb([0, 0, 0]), 10, 10, scale, &font, &final_title);

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
            let text = Self::wrap_text(text, 40);
            imageproc::drawing::draw_text_mut(
                &mut img,
                Rgb([0, 0, 0]),
                10,
                y_position as i32,
                scale,
                &font,
                text.as_str(),
            );
            y_position += scale.y * 1.5; // 행 간격 설정
        }

        if let Some(parent_dir) = std::path::Path::new(file_path).parent() {
            if !parent_dir.exists() {
                match fs::create_dir_all(parent_dir) {
                    Ok(()) => {}
                    Err(_err) => {}
                }
            }
        }

        img.save(file_path).expect("Unable to save image");

        // exif : https://www.awaresystems.be/imaging/tiff/tifftags/privateifd/exif.html
        let mut metadata = Metadata::new();
        metadata.set_tag(
            ExifTag::DateTimeOriginal(file_date.format("%Y-%m-%d %H:%M:%S").to_string()),
        );
        metadata.set_tag(
            ExifTag::DateTimeOriginal(file_date.format("%Y-%m-%d %H:%M:%S").to_string())
        );
        metadata.write_to_file(std::path::Path::new(file_path)).expect("image esif write fail.");

        // 파일 날짜
        let file_time = FileTime::from_unix_time(file_date.timestamp(), 0);
        match filetime::set_file_times(file_path, file_time, file_time) {
            Ok(()) => {}
            Err(_err) => {}
        }
        Ok(())
    }

    pub fn text_to_txt_file(
        title: &str,
        center_name: &Option<String>,
        author_name: &str,
        contents: &Vec<&str>,
        file_path: &str,
        file_date: DateTime<Utc>,
    ) -> Result<(), AuthError> {
        match File::create(file_path) {
            Ok(mut file) => {
                let final_title = match center_name {
                    Some(center_name) 
                        if center_name != author_name 
                        && author_name.find(center_name).is_none() 
                    => {
                        format!("{} ({})\n", title, center_name)
                    }
                    _ => {
                        format!("{}\n", title)
                    }
                };
                if let Err(err) = file.write_all(final_title.as_bytes()){
                    println!("write_all error. {err}");
                }
                if let Err(err) = file.write_all( format!("작성자 : {}\n", author_name).as_bytes()){
                    println!("write_all error. {err}");
                }
                if let Err(err) = file.write_all(b"---\n"){
                    println!("write_all error. {err}");
                }
                for text in contents {
                    let text = Self::wrap_text(text, 40);
                    if let Err(err) = file.write_all(text.as_bytes()){
                        println!("write_all error. {err}");
                    }
                    if let Err(err) = file.write_all(b"\n"){
                        println!("write_all error. {err}");
                    }
                }

                // 파일 날짜
                let file_time = FileTime::from_unix_time(file_date.timestamp(), 0);
                if let Err(err) = filetime::set_file_times(file_path, file_time, file_time){
                    println!("set_file_times error. {err}");
                }
            },
            Err(_e) => {
            }
        };
        Ok(())
    }

    pub fn wrap_text(text: &str, limit: usize) -> String {
        let max = limit + 7;
        let mut result = String::new();
    
        let mut count = 0;
        let chars = text.chars();
        for c in chars {
            count += 1;
            if count >= limit {
                if c == ' ' {
                    result.push('\n');
                    count = 0;
                } else if count >= max {
                    result.push(c);
                    result.push('\n');
                    count = 0;
                } else {
                    result.push(c);
                }
                
            } else {
                result.push(c);
            }
        }
        result
    }
}
