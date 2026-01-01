use std::error;

pub type DynResult<T> = Result<T, Box<dyn error::Error>>;
