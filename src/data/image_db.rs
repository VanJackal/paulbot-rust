use crate::data::pool::Pool;
use crate::error::PaulError;
use crate::objects::{Cat, CatImage};
use crate::settings::Settings;

pub trait ImageDatabase: Send + Sync {
    fn get_image(&self, cat: &Option<Cat>) -> Result<CatImage, PaulError>;
}

#[derive(Clone)]
pub struct ImageDB {
    default_cat_id: i64,
    pool: Pool
}

impl ImageDB {
    pub fn new(settings: & Settings, pool: Pool) -> Self {
        Self::init_db(&pool);
        Self {
            pool,
            default_cat_id:settings.commands.default_cat_id
        }
    }

    fn init_db(pool: &Pool) {
        let conn = pool.get().unwrap();

        if !conn.table_exists(None, "cats").unwrap() {
            let create_query = include_str!("database.sql");
            conn.execute_batch(create_query).unwrap();
        }
    }
}

impl <'a> ImageDatabase for ImageDB {
    fn get_image(&self, cat: &Option<Cat>) -> Result<CatImage, PaulError> {
        let sql = "SELECT images.id, images.url, images.cat_id, cats.name FROM images LEFT JOIN cats ON images.cat_id = cats.id WHERE cat_id = ? ORDER BY random() LIMIT 1;";
        let conn = self.pool.get().unwrap();
        let mut stmt = conn.prepare(sql).unwrap();
        let id:i64;
        if let Some(cat) = cat {
            id = cat.id;
        } else {
            id = self.default_cat_id;
        }
        let rows = stmt.query_map([id],|row| {
            Ok(CatImage {//todo these should be in a separate func
                id: row.get(0)?,
                url: row.get(1)?,
                cat: Cat{
                    id: row.get(2)?,
                    name: row.get(3)?,
                }
            })
        });

        if let Ok(images) = rows {
            for image in images {
                return Ok(image.unwrap());
            }
        }

        Err(PaulError::ImageRetrievalError("Failed to retrieve single image from database.".into()))
    }
}