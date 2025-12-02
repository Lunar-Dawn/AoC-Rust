use std::error;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
pub type Part = fn(&PathBuf) -> Result<String>;
