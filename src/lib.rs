#![allow(unused)]
mod error;
mod prelude;

use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{BufReader, BufWriter, Read, Write},
    path::{self, Path},
};

use crate::prelude::{AppError, Result};

pub trait JsonConverter {
    fn to_json(&self) -> String;
    fn from_json(json: String) -> Self;
}

fn get_dir(identifier: &str) -> String {
    let mut path: String = get_appdata();
    let program_name = env::current_exe()
        .ok()
        .and_then(|path| {
            path.file_stem()
                .map(|os_str| os_str.to_string_lossy().into_owned())
        })
        .unwrap_or_else(|| "default_program".to_string());

    path.push_str("/");
    path.push_str(&program_name);
    path.push_str("/");
    path.push_str(identifier);
    return path;
}

pub fn save(data: impl JsonConverter, identifier: &str) -> Result<String> {
    let data_string = data.to_json();
    let path = get_dir(identifier);
    write_file(data_string, String::from(path.clone()));
    Ok(path)
}

fn write_file(data: String, file_name: String) -> Result<File> {
    if let Some(dir) = Path::new(&file_name).parent() {
        if !dir.exists() {
            fs::create_dir_all(dir).map_err(|e| AppError {
                code: 5,
                message: format!("Failed to create directory: {}", e),
            })?;
        }
    }
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
    Ok(file)
}

pub fn read(identifier: &str) -> Result<String> {
    let file_name = get_dir(identifier);
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

    Ok(contents)
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
