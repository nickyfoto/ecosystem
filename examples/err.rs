use std::fs;

use anyhow::Context;
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[allow(dead_code)]
    #[error("Error: {0:?}")]
    BigError(Box<BigError>),

    #[error("Custom error: {0}")]
    Custom(String),
}

#[allow(unused)]
#[derive(Debug)]
pub struct BigError {
    a: String,
    b: Vec<String>,
    c: [u8; 1024],
    d: u64,
}

fn main() -> Result<(), anyhow::Error> {
    println!("size of anyhow::Error: {}", size_of::<anyhow::Error>());
    println!("size of std::io::Error: {}", size_of::<std::io::Error>());

    println!(
        "size of std::num::ParseIntError: {}",
        size_of::<std::num::ParseIntError>()
    );

    println!(
        "size of serde_json::Error: {}",
        size_of::<serde_json::Error>()
    );

    println!("size of string is: {}", size_of::<String>());
    println!("size of MyError is: {}", size_of::<MyError>());

    let filename = "non-existent-file.txt";
    //  `?` works because of Io(#[from] std::io::Error),
    // with_context is more efficient than context
    let _fd =
        fs::File::open(filename).with_context(|| format!("Failed to open file: {}", filename))?;

    fail_with_error()?;

    Ok(())
}

fn fail_with_error() -> Result<(), MyError> {
    Err(MyError::Custom("This is a custom error".to_string()))
}
