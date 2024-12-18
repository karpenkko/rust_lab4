// use sqlite::Connection;
// use argon2::{
//     password_hash::{
//         rand_core::OsRng,
//         PasswordHash, 
//         PasswordHasher, 
//         PasswordVerifier, 
//         SaltString
//     },
//     Argon2
// };

// pub fn init_db() -> Connection {
//     let conn = sqlite::open("chat.db").unwrap();
//     conn.execute(
//         "
//         CREATE TABLE IF NOT EXISTS users (
//             id INTEGER PRIMARY KEY AUTOINCREMENT,
//             username TEXT UNIQUE,
//             password_hash TEXT
//         );
        
//         CREATE TABLE IF NOT EXISTS messages (
//             id INTEGER PRIMARY KEY AUTOINCREMENT,
//             username TEXT,
//             message TEXT,
//             timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
//         );
//         ",
//     )
//     .unwrap();
//     conn
// }

// pub fn register_user(conn: &Connection, username: &str, password: &str) -> bool {
//     let password_hash = hash_password(password);
//     conn.execute(format!(
//         "INSERT INTO users (username, password_hash) VALUES ('{}', '{}')",
//         username, password_hash
//     ))
//     .is_ok()
// }

// fn hash_password(password: &str) -> String {
//     let salt = SaltString::generate(&mut OsRng);
//     let argon2 = Argon2::default();
    
//     argon2.hash_password(password.as_bytes(), &salt)
//         .unwrap()
//         .to_string()
// }

// pub fn verify_user(conn: &Connection, username: &str, password: &str) -> bool {
//     let mut statement = conn
//         .prepare("SELECT password_hash FROM users WHERE username = ?")
//         .unwrap();
//     statement.bind((1, username)).unwrap();

//     if let Ok(sqlite::State::Row) = statement.next() {
//         let stored_hash: String = statement.read(0).unwrap();
        
//         let parsed_hash = PasswordHash::new(&stored_hash).unwrap();
//         Argon2::default()
//             .verify_password(password.as_bytes(), &parsed_hash)
//             .is_ok()
//     } else {
//         false
//     }
// }


// pub fn save_message(conn: &Connection, username: &str, message: &str) {
//     conn.execute(format!(
//         "INSERT INTO messages (username, message) VALUES ('{}', '{}')",
//         username, message
//     ))
//     .unwrap();
// }

// pub fn get_chat_history(conn: &Connection) -> Vec<(String, String, String)> {
//     let mut statement = conn
//         .prepare("SELECT username, message, timestamp FROM messages ORDER BY timestamp ASC")
//         .unwrap();
    
//     let mut messages = Vec::new();
//     while let Ok(sqlite::State::Row) = statement.next() {
//         let username: String = statement.read(0).unwrap();
//         let message: String = statement.read(1).unwrap();
//         let timestamp: String = statement.read(2).unwrap();
//         messages.push((username, message, timestamp));
//     }
//     messages
// }
