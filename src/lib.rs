#![allow(unused)]
mod error;
mod prelude;

mod json_struct_db {
    use std::{
        fs::{File, OpenOptions},
        io::{BufReader, BufWriter, Read, Write},
        path::Path,
    };

    use crate::prelude::{AppError, Result};

    pub trait JsonConverter {
        fn to_json(&self) -> String;
        fn from_json(json: String) -> Self;
    }

    pub fn save(data: impl JsonConverter, identifier: &str) -> Result<()> {
        let data_string = data.to_json();
        let mut path: String = get_appdata();
        path.push_str(identifier);
        write_file(data_string, String::from(identifier))
    }

    fn write_file(data: String, file_name: String) -> Result<()> {
        let mut file = match OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_name)
        {
            Ok(file) => file,
            Err(e) => {
                return Err(AppError {
                    code: 2,
                    message: format!("Failed to open File: {}", e),
                })
            }
        };
        file.write_all(data.as_bytes()).map_err(|e| AppError {
            code: 3,
            message: format!("Failed to write to file: {}", e),
        })?;
        Ok(())
    }

    pub fn read<T: JsonConverter>(identifier: &str) -> Result<T> {
        let mut file_name = get_appdata();
        file_name.push_str(identifier);
        let file = match File::open(&file_name) {
            Ok(file) => file,
            Err(e) => {
                return Err(AppError {
                    code: 2,
                    message: format!("Failed to open File: {}", e),
                })
            }
        };

        let mut contents = String::new();
        let mut reader = BufReader::new(file);
        reader.read_to_string(&mut contents).map_err(|e| AppError {
            code: 4,
            message: format!("Failed to read from file: {}", e),
        })?;

        let data = T::from_json(contents);
        Ok(data)
    }

    fn get_appdata() -> String {
        match dirs::data_dir() {
            None => return String::new(),
            Some(dir) => match dir.to_str() {
                None => return String::new(),
                Some(d) => return String::from(d),
            },
        };
    }
}
