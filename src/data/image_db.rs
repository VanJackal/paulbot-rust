use crate::error::PaulError;
use crate::objects::{Cat, CatImage};
use sqlite;
use sqlite::{Connection, State};
use crate::settings::Settings;

pub trait ImageDatabase {
    fn get_image(&self, cat: &Option<Cat>) -> Result<CatImage, PaulError>;
}

pub struct ImageDB{
    connection: Connection,
    default_cat_id: i64
}

impl ImageDB {
    pub fn new(settings: & Settings) -> Self {
        let conn = sqlite::open(settings.database.url.clone()).unwrap();
        Self::init_db(&conn);
        Self {
            connection:conn,
            default_cat_id:settings.commands.default_cat_id
        }
    }

    fn init_db(conn: &Connection) {
        let check_query = "SELECT name FROM sqlite_master WHERE type='table' AND name='cats'";
        let mut count = 0;
        conn.iterate(check_query, |_| {
            count += 1;
            true
        }).unwrap();

        if count == 0 {
            let create_query = include_str!("database.sql");
            conn.execute(create_query).unwrap();
        }
    }
}

impl ImageDatabase for ImageDB {
    fn get_image(&self, cat: &Option<Cat>) -> Result<CatImage, PaulError> {
        let sql = "SELECT * FROM images WHERE cat_id = ? ORDER BY random() LIMIT 1;";
        let mut stmt = self.connection.prepare(sql).unwrap();
        let id:i64;
        if let Some(cat) = cat {
            id = cat.id;
        } else {
            id = self.default_cat_id;
        }
        stmt.bind((1, id)).unwrap();
        if let Ok(State::Row) = stmt.next() {
            return Ok(CatImage { //todo cat should be retrieved in the same query
                id: stmt.read::<i64,_>("id").unwrap(),
                url: stmt.read::<String,_>("url").unwrap(),
                cat: Cat{
                    name:"a cat".into(),
                    id: 1,

                }
            }) //todo this conversion logic should be done in a separate file
        }
        Err(PaulError::ImageRetrievalError("Failed to retrieve single image from database.".into()))
    }
}