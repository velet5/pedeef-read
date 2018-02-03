use std;
use reader::error::ReadError;

pub type ReadResult<T> = std::result::Result<T, ReadError>;