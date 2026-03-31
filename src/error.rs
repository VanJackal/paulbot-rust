use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::objects::Cat;

#[derive(Debug)]
pub enum PaulError {
    InvalidCat(Cat)
}

impl Display for PaulError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PaulError::InvalidCat(cat) => write!(f, "Invalid cat: {}", cat.name),
        }
    }
}

impl Error for PaulError {}
