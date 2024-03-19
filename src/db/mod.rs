use crate::auth::user::User;
use rusqlite::Connection;

pub struct Database {
    conn: Connection,
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}

impl Database {
    pub fn new() -> Self {
        let conn = Connection::open_in_memory().unwrap();
        Database { conn }
    }

    pub fn create_user(&self, user: User) -> bool {
        match self.conn.execute(
            "INSERT INTO users(username, password) VALUES (?1, ?2)",
            (&user.username, &user.password),
        ) {
            Err(error) => {
                println!("Problem Creating New User: {:#?}", error);
                false
            }
            Ok(resp) => {
                println!("New User Created: {}", resp); // just for now
                true
            }
        }
    }
}
