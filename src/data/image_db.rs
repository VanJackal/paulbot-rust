use crate::error::PaulError;
use crate::objects::{Cat, CatImage};

pub trait ImageDatabase: Send + Sync {
    fn get_image(&self, cat: Option<Cat>) -> Result<CatImage, PaulError>;
}

struct ImageDB {}