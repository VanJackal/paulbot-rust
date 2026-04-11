use r2d2_sqlite::SqliteConnectionManager;
use crate::settings::Settings;

pub fn get_pool(settings: &Settings) -> Pool {
    let manager:SqliteConnectionManager;
    if settings.database.url == ":memory:" {
        manager = SqliteConnectionManager::memory();
    } else {
        manager = SqliteConnectionManager::file(&settings.database.url);
    }
    r2d2::Pool::builder().build(manager).unwrap()
}

pub type Pool = r2d2::Pool<SqliteConnectionManager>;
