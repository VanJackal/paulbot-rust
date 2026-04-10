use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::objects::Cat;

#[derive(Debug)]
pub enum PaulError {
    InvalidCat(Cat),
    ImageRetrievalError(String),
}

impl Display for PaulError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PaulError::InvalidCat(cat) => write!(f, "Invalid cat: {}", cat.name),
            PaulError::ImageRetrievalError(msg) => write!(f, "Image retrieval error: {}", msg),
        }
    }
}

impl Error for PaulError {}
