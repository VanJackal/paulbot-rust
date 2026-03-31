use crate::data::ImageDatabase;
use crate::error::PaulError;
use crate::objects::{Cat, CatImage};
use crate::settings::Settings;

pub trait ImageProvider {
    fn get_image(&self, cat:Option<Cat>) -> Result<CatImage, PaulError>;
}


struct ImageManager {
    db: Box<dyn ImageDatabase>,
    settings: Settings
}

impl ImageProvider for ImageManager {
    fn get_image(&self, cat: Option<Cat>) -> Result<CatImage, PaulError> {
        self.db.get_image(cat)
    }
}




#[cfg(test)]
mod test {
    use crate::data::ImageDatabase;
    use crate::error::PaulError;
    use crate::logic::image_manager::ImageManager;
    use crate::logic::ImageProvider;
    use crate::objects::{Cat, CatImage};
    use crate::settings::Settings;

    struct FakeDB {
        image: CatImage
    }

    impl ImageDatabase for FakeDB {
        fn get_image(&self, cat: Option<Cat>) -> Result<CatImage, PaulError> {

            if let Some(c) = cat{
                if c.id == self.image.cat.id {
                    Ok(self.image.clone())
                } else {
                    Err(PaulError::InvalidCat(c))
                }
            } else {
                Ok(self.image.clone())
            }

        }
    }

    fn setup() -> (ImageManager, CatImage, Cat) {
        let cat = Cat{
            id: 0,
            name: "A Cat".to_string(),
        };
        let image = CatImage {
            id: 0,
            url: "url.to.an.image".to_string(),
            cat: cat.clone(),
        };
        (
            ImageManager {
                db: Box::new(FakeDB{image:image.clone()}),
                settings: Settings::new().unwrap(),
            },
            image,
            cat

        )
    }

    #[test]
    fn test_valid_cat() {
        let (im, image, cat) = setup();

        let result = im.get_image(Some(cat));
        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, image.id);
    }

    #[test]
    fn test_invalid_cat() {
        let (im, _image, _cat) = setup();
        let invalid_cat = Cat{
            id: 1337,
            name: "A Cat".to_string(),
        };

        let result = im.get_image(Some(invalid_cat));
        assert!(result.is_err());
    }

    #[test]
    fn test_test_no_cat() {
        let (im, image, _cat) = setup();

        let result = im.get_image(None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, image.id);
    }
}