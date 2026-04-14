use paulbot_rust::logic::ImageProvider;
pub mod common;
use common::settings;

use paulbot_rust::data::ImageDB;
use paulbot_rust::data::pool::get_pool;
use paulbot_rust::logic::ImageManager;
use paulbot_rust::objects::Cat;
use paulbot_rust::settings::Settings;

struct Env {
    pub db: ImageDB,
    pub settings: Settings,
    pub cat: Cat
}


impl Env {
    fn new() -> Self {
        let settings = settings();
        let db = ImageDB::new(&settings, get_pool(&settings));
        Self {
            settings:settings.clone(),
            db,
            cat: Cat {
                id: settings.commands.default_cat_id,
                name: "A Cat".to_string()
            }
        }
    }

    fn im(&'_ self) -> ImageManager {
        ImageManager::new(self.settings.clone(), Box::new(self.db.clone()))
    }

}


#[test]
fn test_get_image_default() {
    let env = Env::new();

    let image = env.im().get_image(&None).unwrap();
    assert_eq!(image.cat.id, env.settings.commands.default_cat_id);
}

#[test]
fn test_get_image_cat() {
    let env = Env::new();

    let image = env.im().get_image(&Some(env.cat.clone())).unwrap();
    assert_eq!(image.cat.id, env.cat.id);

}

#[test]
fn test_invalid_cat() {
    let env = Env::new();

    let res = env.im().get_image(&Some(Cat {id:0, name:"A name".into()}));
    assert!(res.is_err());
}