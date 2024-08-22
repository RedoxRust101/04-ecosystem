use std::mem::size_of;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
  #[error("I/O error: {0}")]
  Io(#[from] std::io::Error),
  #[error("Parse int error: {0}")]
  ParseInt(#[from] std::num::ParseIntError),
  #[error("Serialize error: {0}")]
  Serialize(#[from] serde_json::Error),
  #[error("Big error: {0:?}")]
  BigError(Box<BigError>),
  #[error("Custom error: {0}")]
  Custom(String),
}
#[allow(unused)]
#[derive(Debug)]
pub struct BigError {
  a: String,
  b: Vec<String>,
  c: [u8; 64],
  d: u64,
}

fn main() -> Result<(), anyhow::Error> {
  println!("size of anyhow::Error is {}", size_of::<anyhow::Error>());
  println!("size of io::Error is {}", size_of::<std::io::Error>());
  println!("size of std::num::ParseIntError is {}", size_of::<std::num::ParseIntError>());
  println!("size of serde_json::Error is {}", size_of::<serde_json::Error>());
  println!("size of String is {}", size_of::<String>());
  println!("size of MyError is {}", size_of::<MyError>());

  /*   let filename = "non-existent-file.txt";
   let _fd =
     fs::File::open(filename).with_context(|| format!("Failed to open file {}", filename))?;
  */
  fail_with_error()?;

  Ok(())
}

fn fail_with_error() -> Result<(), MyError> {
  Err(MyError::Custom("This is a custom error".to_string()))
}
