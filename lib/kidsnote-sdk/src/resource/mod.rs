use std::{sync::{Mutex, Arc}, io::Write, fs::{self, File}, time::Duration};

use crate::{options::KidsnoteOptions, auth::error_types::AuthError};

pub mod datatypes;

pub struct KidsnoteResourceSdk {
    //options: Arc<Mutex<KidsnoteOptions>>,
}

impl KidsnoteResourceSdk {
    pub fn new(_options:Arc<Mutex<KidsnoteOptions>>) -> KidsnoteResourceSdk {
        Self { 
            //options
        }
    }

    pub async fn download_image(&self, url:&str, file_size:i32, download_path:&str) -> Result<bool, AuthError> {
       
        // if let Ok(mut ouput_file) = fs::OpenOptions::new()
        //     .append(true)
        //     .create(true)
        //     .open("download_history.txt") 
        // {
        //     let text = format!("{},{}\n", url, download_path);
        //     match ouput_file.write_all(text.as_bytes()) {
        //         Ok(_) => return Ok(true),
        //         Err(err) => return Err(AuthError::GeneralErrorStr(format!("file error. {:?}", err)))
        //     }
        // }

        if let Some(parent_dir) = std::path::Path::new(download_path).parent() {
            if !parent_dir.exists() {
                fs::create_dir_all(parent_dir)
                    .map_err(|err| AuthError::GeneralErrorStr(format!("Failed to create directory: {}", err)))?;
            }
        }

        match fs::metadata(&download_path) {
            Ok(metadata) => {
                if metadata.len() == file_size as u64 {
                    return Ok(false);
                }
            },
            Err(err) => {
                match err.kind() {
                    std::io::ErrorKind::NotFound => {},
                    _=> {
                        return Err(AuthError::GeneralErrorStr(format!("file metadata error. {:?}", err)));
                    }
                }
            }
        }

        for _ in 0..3 {
            let client = crate::common::get_client();
            match client.get(url)
                .timeout(Duration::from_secs(5))
                .send().await 
            {
                Ok(response) => {
                    match response.bytes().await {
                        Ok(bytes) => {
                            let mut output_file = File::create(download_path)
                                .map_err(|err| AuthError::GeneralErrorStr(format!("File open error. path={}, {}", download_path, err)))?;
                            output_file
                                .write_all(&bytes)
                                .map_err(|err| AuthError::GeneralErrorStr(format!("Error writing to file: {}", err)))?;
                            return Ok(true);
                        },
                        Err(err) => {
                            println!("error. {}", err);
                            return Err(AuthError::GeneralErrorStr(format!("unknown error. {:?}", err)));
                        }
                    }
                },
                Err(err) => {
                    println!("error. {}", err);
                    return Err(AuthError::GeneralErrorStr(format!("unknown error. {:?}", err)));
                }
            }
        }

        println!("error. no call");
        Err(AuthError::GeneralErrorStr(format!("unknown error. no call")))
    }
}