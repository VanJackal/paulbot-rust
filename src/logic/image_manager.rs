use crate::data::ImageDatabase;
use crate::error::PaulError;
use crate::objects::{Cat, CatImage};
use crate::settings::Settings;

pub trait ImageProvider {
    fn get_image(&self, cat:Option<Cat>) -> Result<CatImage, PaulError>;
}


struct ImageManager<'a> {
    db: &'a (dyn ImageDatabase + 'a),
    settings: Settings
}

impl ImageProvider for ImageManager<'_> {
    fn get_image(&self, cat: Option<Cat>) -> Result<CatImage, PaulError> {
        self.db.get_image(cat)
    }
}

impl<'a> ImageManager<'a> {
    fn new(settings: Settings, db: &'a impl ImageDatabase) -> Self {
        Self { db, settings }
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

    struct Env {
        pub db:FakeDB,
        pub image: CatImage,
        pub cat: Cat
    }

    impl Env{
        fn new() -> Self {
            let cat = Cat{
                id: 0,
                name: "A Cat".to_string(),
            };
            let image = CatImage {
                id: 0,
                url: "url.to.an.image".to_string(),
                cat: cat.clone(),
            };
            let db = FakeDB { image: image.clone() };

            Self {
                db,
                image,
                cat
            }
        }

        fn get(&'_ self) -> (ImageManager<'_>, CatImage, Cat) {
            (
                ImageManager::new(Settings::new().unwrap(),&self.db),
                self.image.clone(),
                self.cat.clone()
            )
        }
    }




    #[test]
    fn test_valid_cat() {
        let env = Env::new();
        let (im, image, cat) = env.get();

        let result = im.get_image(Some(cat));
        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, image.id);
    }

    #[test]
    fn test_invalid_cat() {
        let env = Env::new();
        let (im, _image, _cat) = env.get();

        let invalid_cat = Cat{
            id: 1337,
            name: "A Cat".to_string(),
        };

        let result = im.get_image(Some(invalid_cat));
        assert!(result.is_err());
    }

    #[test]
    fn test_test_no_cat() {
        let env = Env::new();
        let (im, image, _cat) = env.get();

        let result = im.get_image(None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, image.id);
    }
}