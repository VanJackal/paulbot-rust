#[derive(Debug, Clone)]
pub struct CatImage {
    pub id: i64,
    pub url: String,
    pub cat: Cat,
}

#[derive(Debug, Clone)]
pub struct Cat {
    pub id: i64,
    pub name: String
}