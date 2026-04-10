use paulbot_rust::logic::ImageProvider;
pub mod common;
use common::settings;

use paulbot_rust::data::ImageDB;
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
        Self {
            db: ImageDB::new(&settings),
            settings:settings.clone(),
            cat: Cat {
                id: settings.commands.default_cat_id,
                name: "A Cat".to_string()
            }
        }
    }
    fn get_im(&'_ self) -> ImageManager<'_> {
        ImageManager::new(self.settings.clone(), &self.db)
    }
}


#[test]
fn test_get_image_default() {
    let env = Env::new();
    let im = env.get_im();

    let image = im.get_image(&None).unwrap();
    assert_eq!(image.cat.id, env.settings.commands.default_cat_id);
}

#[test]
fn test_get_image_cat() {
    let env = Env::new();
    let im = env.get_im();

    let image = im.get_image(&Some(env.cat.clone())).unwrap();
    assert_eq!(image.cat.id, env.cat.id);

}

#[test]
fn test_invalid_cat() {
    let env = Env::new();
    let im = env.get_im();

    let res = im.get_image(&Some(Cat {id:0, name:"A name".into()}));
    assert!(res.is_err());
}