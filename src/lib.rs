use anyhow::Result;

mod error;
pub mod meta_command;
pub mod statement;

#[derive(Debug)]
pub struct User {
    id: i32,
    username: [char; 32],
    email: [char; 255],
}

impl User {
    pub fn new(id: &str, username_string: &str, email_string: &str) -> Result<Self> {
        let mut username: [char; 32] = ['\0'; 32];
        for (i, c) in username_string.chars().take(32).enumerate() {
            username[i] = c;
        }
        let mut email: [char; 255] = ['\0'; 255];
        for (i, c) in email_string.chars().take(32).enumerate() {
            email[i] = c;
        }

        Ok(User {
            id: id.parse()?,
            username: username,
            email: email,
        })
    }
}
