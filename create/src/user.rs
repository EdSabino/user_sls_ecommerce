use rust_sls_common::{ Error, generic_error::GenericError, serde_date};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::str;
use regex::Regex;

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub email: String,
    pub password: String,
    pub name: String,
    #[serde(with = "serde_date")]
    pub birthday: DateTime<Utc>
}

impl User {
    pub fn validate(&self) -> Result<(), Error>{
        let re = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})")?;
        if !re.is_match(self.email.clone().as_str()) {
            return Err(Box::new(GenericError { description: "invalid_email".to_string() }));
        }
        if self.password.chars().count() < 4 {
            return Err(Box::new(GenericError { description: "password_to_short".to_string() }));
        }
        Ok(())
    }

    pub fn hash_password(&mut self) {
        let mut hasher = Sha256::new();

        hasher.update(self.password.clone());

        let result = hasher.finalize();
        self.password = format!("{:x}", result);
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn user_email_must_be_valid() {
        let user = User {
            email: "email".to_string(),
            password: "password".to_string(),
            name: "Usuario".to_string(),
            birthday: Utc::now()
        };
        match user.validate() {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(err.to_string(), "invalid_email")
        }

        let user = User {
            email: "email.com".to_string(),
            password: "password".to_string(),
            name: "Usuario".to_string(),
            birthday: Utc::now()
        };
        match user.validate() {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(err.to_string(), "invalid_email")
        }

        let user = User {
            email: "email@email.com".to_string(),
            password: "password".to_string(),
            name: "Usuario".to_string(),
            birthday: Utc::now()
        };
        match user.validate() {
            Ok(_) => assert!(true),
            Err(_) => assert!(false)
        }
    }

    #[tokio::test]
    async fn user_password_must_have_more_then_4_characters() {
        let user = User {
            email: "email@email.com".to_string(),
            password: "pas".to_string(),
            name: "Usuario".to_string(),
            birthday: Utc::now()
        };
        match user.validate() {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(err.to_string(), "password_to_short")
        }

        let user = User {
            email: "email@email.com".to_string(),
            password: "pass".to_string(),
            name: "Usuario".to_string(),
            birthday: Utc::now()
        };
        match user.validate() {
            Ok(_) => assert!(true),
            Err(_) => assert!(false)
        }

        let user = User {
            email: "email@email.com".to_string(),
            password: "password".to_string(),
            name: "Usuario".to_string(),
            birthday: Utc::now()
        };
        match user.validate() {
            Ok(_) => assert!(true),
            Err(_) => assert!(false)
        }
    }

    #[tokio::test]
    async fn must_hash_password_with_sh2() {
        let mut user = User {
            email: "email@email.com".to_string(),
            password: "password".to_string(),
            name: "Usuario".to_string(),
            birthday: Utc::now()
        };
        user.hash_password();

        let mut hasher = Sha256::new();
        hasher.update("password");
        let result = hasher.finalize();
        assert_eq!(user.password, format!("{:x}", result));
    }
}