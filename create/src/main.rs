mod user;
mod database;

use database::DatabaseName;
use lambda::{handler_fn, Context};
use serde_json::{Value, json};
use mongodb::{Client, bson::to_document};
use rust_sls_common::{Event, Error};
use user::User;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler_fn(create)).await?;
    Ok(())
}

async fn create(event: Value, _: Context) -> Result<Value, Error> {
    let client =  rust_sls_common::create_client().await?;
    let parsed_event: Event = serde_json::from_value(event)?;
    match create_user(client, serde_json::from_value(parsed_event.body)?).await {
        Ok(_) => Ok(json!({ "statusCode": 204 })),
        Err(err) => Ok(json!({
            "statusCode": 400,
            "body": json!({ "message": format!("{:?}", err)})
        }))
    }
}

async fn create_user(client: Client, mut user: User) -> Result<(), Error> {
    user.validate()?;
    user.hash_password();
    let db = client.database(&DatabaseName::get_database_name().as_str());
    let collection = db.collection("users");
    collection.insert_one(to_document(&user).unwrap(), None).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::env;

    #[tokio::test]
    async fn must_fail_user_null() {
        env::set_var("MONGODBENDPOINT", "mongodb+srv://adm:9XIh4bdywjikBOKu@development.khj7l.mongodb.net/users_test?retryWrites=true&w=majority");
        let event = json!({
            "resource": "/",
            "path": "/",
            "httpMethod": "POST",
            "queryStringParameters": "",
            "body": ""
        });
        match create(event.clone(), Context::default()).await {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(err.to_string(), "invalid type: string \"\", expected struct User")
        }
    }

    #[tokio::test]
    async fn user_must_have_email() {
        env::set_var("MONGODBENDPOINT", "mongodb+srv://adm:9XIh4bdywjikBOKu@development.khj7l.mongodb.net/users_test?retryWrites=true&w=majority");
        let event = json!({
            "resource": "/",
            "path": "/",
            "httpMethod": "POST",
            "queryStringParameters": "",
            "body": json!({
                "password": "1234"
            })
        });
        match create(event.clone(), Context::default()).await {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(err.to_string(), "missing field `email`")
        }
    }

    #[tokio::test]
    async fn user_must_have_password() {
        env::set_var("MONGODBENDPOINT", "mongodb+srv://adm:9XIh4bdywjikBOKu@development.khj7l.mongodb.net/users_test?retryWrites=true&w=majority");
        let event = json!({
            "resource": "/",
            "path": "/",
            "httpMethod": "POST",
            "queryStringParameters": "",
            "body": json!({
                "email": "email@gmail.com"
            })
        });
        match create(event.clone(), Context::default()).await {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(err.to_string(), "missing field `password`")
        }
    }

    #[tokio::test]
    async fn user_must_be_created() {
        env::set_var("MONGODBENDPOINT", "mongodb+srv://adm:9XIh4bdywjikBOKu@development.khj7l.mongodb.net/users_test?retryWrites=true&w=majority");
        let event = json!({
            "resource": "/",
            "path": "/",
            "httpMethod": "POST",
            "queryStringParameters": "",
            "body": json!({
                "email": "email@email.com",
                "password": "password"
            })
        });
        match create(event.clone(), Context::default()).await {
            Ok(val) => assert_eq!(json!({ "statusCode": 204 }), val),
            Err(_) => assert!(false)
        }
    }
}
