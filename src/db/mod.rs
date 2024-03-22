use crate::auth::{documents::AllowedDocuments, hash_string, user::User};
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
        let conn = Connection::open("./cdn.db").unwrap();
        let user_table = conn.execute(
            "CREATE TABLE IF NOT EXISTS users(
                username TEXT NOT NULL,
                password TEXT NOT NULL,
                allowed_documents BLOB NULL
            );",
            [],
        );

        println!(
            "{} USER TABLE CREATED: {:?}",
            "[DATABASE]:".yellow(),
            user_table.unwrap() == 0
        );

        Database { conn }
    }

    pub fn create_user(&self, user: User) -> Result<usize, rusqlite::Error> {
        // Check if the user already exists
        let user_exists: bool = match self.conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM users WHERE username = ?1)",
            &[&user.username],
            |row| row.get(0),
        ) {
            Ok(exists) => exists,
            Err(_) => false, // Assume user doesn't exist if there's an error
        };

        // If user already exists, return an error
        if user_exists {
            return Err(rusqlite::Error::QueryReturnedNoRows); // You can replace this with an appropriate error
        }

        let hashed_pass = hash_string(user.password.as_str()).unwrap();
        // Attempt to insert a new user into the database
        match self.conn.execute(
            "INSERT INTO users(username, password) VALUES (?1, ?2)",
            (&user.username, &hashed_pass),
        ) {
            Err(error) => {
                println!("Problem Creating New User: {:#?}", error);
                Err(error)
            }
            Ok(resp) => Ok(resp),
        }
    }

    pub fn user_exists(&self, username: &str) -> Result<bool, rusqlite::Error> {
        // Execute a SQL query to check if a user with the given username exists
        let result = self
            .conn
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM users WHERE username = ?1)",
                &[username],
                |row| row.get(0),
            )
            .unwrap_or(None); // Handle the case where the query returns no rows

        // Convert the result to a boolean and return
        Ok(result.unwrap_or(false))
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
