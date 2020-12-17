use rust_sls_common::{ Error, generic_error::GenericError, serde_date };
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use serde_json::Value;
use chrono::NaiveDate;
use std::str;
use regex::Regex;

#[derive(Deserialize, Debug, Serialize)]
pub struct User {
    pub email: String,
    pub password: String,
    pub name: String,
    #[serde(with = "serde_date")]
    pub birthday: NaiveDate,
    pub document: String,
    pub extra: Option<Value>
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

    pub trait Mock {
        fn mock() -> Self;
    }

    impl Mock for User {
        fn mock() -> Self {
            User {
                email: "email@email.com".to_string(),
                password: "password".to_string(),
                name: "Usuario".to_string(),
                birthday: NaiveDate::from_ymd(1999, 06, 21),
                document: "00000000000".to_string(),
                extra: None
            }
        }
    }

    #[tokio::test]
    async fn user_email_must_be_valid() {
        let mut user = User::mock();
        user.email = "email".to_string();
        match user.validate() {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(err.to_string(), "invalid_email")
        }

        let mut user = User::mock();
        user.email = "email.com".to_string();
        match user.validate() {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(err.to_string(), "invalid_email")
        }

        let user = User::mock();
        match user.validate() {
            Ok(_) => assert!(true),
            Err(_) => assert!(false)
        }
    }

    #[tokio::test]
    async fn user_password_must_have_more_then_4_characters() {
        let mut user = User::mock();
        user.password = "pas".to_string();
        match user.validate() {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(err.to_string(), "password_to_short")
        }

        let mut user = User::mock();
        user.password = "pass".to_string();
        match user.validate() {
            Ok(_) => assert!(true),
            Err(_) => assert!(false)
        }

        let user = User::mock();
        match user.validate() {
            Ok(_) => assert!(true),
            Err(_) => assert!(false)
        }
    }

    #[tokio::test]
    async fn must_hash_password_with_sh2() {
        let mut user = User::mock();
        user.hash_password();

        let mut hasher = Sha256::new();
        hasher.update("password");
        let result = hasher.finalize();
        assert_eq!(user.password, format!("{:x}", result));
    }
}