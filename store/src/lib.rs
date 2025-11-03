use diesel::{ Connection, ConnectionError, PgConnection};

use crate::config::Config;
pub mod schema;
pub mod config;


pub struct Store {
    conn: PgConnection
}

impl Store{
   fn default() -> Result<Self, ConnectionError> {
       let config: Config=Config::default();
       let conn = PgConnection::establish(&config.db_url)?;

       Ok(Self { conn })
   }
}

impl Store {
    pub fn create_user(&self){
        // self.conn.execute("InSererer")
    }

    pub fn create_website(&self)->String{
       String::from('1')
    }
}
