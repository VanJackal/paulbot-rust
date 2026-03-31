#[derive(Debug, Clone)]
pub struct CatImage {
    pub id: u64,
    pub url: String,
    pub cat: Cat,
}

#[derive(Debug, Clone)]
pub struct Cat {
    pub id: u64,
    pub name: String
}