use std::env;

pub struct Config {
    pub db_url: String
}

impl Default for Config {
    fn default() -> Self {
        let db_url = env::var("DATABASE_URl").unwrap_or_else(|_| panic!("Please provide database url environment "));
        Self { db_url }
    }
}
