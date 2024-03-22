use crate::auth::{documents::AllowedDocuments, user::User};
use colored::Colorize;
use rusqlite::Connection;

#[derive()]
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
        let user_table = conn.execute(
            "CREATE TABLE IF NOT EXISTS users(
                username TEXT NOT NULL,
                password TEXT NOT NULL,
                allowed_documents BLOB NULL
            );",
            [],
        );

        println!(
            "{} TABLE CREATED: {:?}",
            "[DATABASE]:".yellow(),
            user_table.unwrap() == 0
        );

        Database { conn }
    }

    pub fn create_user(&self, user: User) -> Result<usize, rusqlite::Error> {
        // Attempt to insert a new user into the database
        match self.conn.execute(
            "INSERT INTO users(username, password) VALUES (?1, ?2)",
            (&user.username, &user.password),
        ) {
            Err(error) => {
                println!("Problem Creating New User: {:#?}", error);
                Err(error)
            }
            Ok(resp) => {
                println!("New User Created: {}", resp); // just for now
                Ok(resp)
            }
        }
    }

    pub fn get_by_username(&self, username: &str) -> Option<User> {
        match self.conn.query_row(
            "SELECT * FROM users WHERE username = ?1",
            [username],
            |row| {
                let username: String = row.get(0)?;
                let password: String = row.get(1)?;
                Ok(User {
                    username,
                    password,
                    allowed_docs: AllowedDocuments::new(),
                })
            },
        ) {
            Ok(user) => Some(user),
            Err(_) => None,
        }
    }
}
