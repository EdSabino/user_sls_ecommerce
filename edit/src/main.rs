mod user;
mod database;
mod parameters;

use database::DatabaseName;
use lambda::{handler_fn, Context};
use serde_json::{Value, json};
use mongodb::{Client, bson::doc, bson::to_document, options::UpdateOptions};
use rust_sls_common::{Event, Error};
use user::User;
use parameters::Parameters;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler_fn(edit)).await?;
    Ok(())
}

async fn edit(event: Value, _: Context) -> Result<Value, Error> {
    let client =  rust_sls_common::create_client().await?;
    let parsed_event: Event = serde_json::from_value(event)?;
    match edit_user(client, serde_json::from_value(parsed_event.body.unwrap())?, serde_json::from_value(parsed_event.path_parameters.unwrap())?).await {
        Ok(_) => Ok(json!({ "statusCode": 204 })),
        Err(err) => Ok(json!({
            "statusCode": 400,
            "body": json!({ "message": format!("{:?}", err)})
        }))
    }
}

async fn edit_user(client: Client, user: User, id: Parameters) -> Result<(), Error> {
    let db = client.database(&DatabaseName::get_database_name().as_str());
    let collection = db.collection("users");
    collection.update_one(doc! {
        "_id": id.user_id
    }, doc! {
        "extra": to_document(&user.extra).unwrap()
    }, UpdateOptions::builder()
        .upsert(true)
        .build()
    ).await?;
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
            "httpMethod": "PUT",
            "queryStringParameters": "",
            "body": ""
        });
        match edit(event.clone(), Context::default()).await {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(err.to_string(), "invalid type: string \"\", expected struct User")
        }
    }

    #[tokio::test]
    async fn user_must_have_password() {
        env::set_var("MONGODBENDPOINT", "mongodb+srv://adm:9XIh4bdywjikBOKu@development.khj7l.mongodb.net/users_test?retryWrites=true&w=majority");
        let event = json!({
            "resource": "/5fd80e0300e4c6ee00a1ade2",
            "path": "/",
            "httpMethod": "PUT",
            "queryStringParameters": "",
            "body": json!({})
        });
        match edit(event.clone(), Context::default()).await {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(err.to_string(), "missing field `extra`")
        }
    }

    #[tokio::test]
    async fn user_must_edit() {
        env::set_var("MONGODBENDPOINT", "mongodb+srv://adm:9XIh4bdywjikBOKu@development.khj7l.mongodb.net/users_test?retryWrites=true&w=majority");
        let event = json!({
            "resource": "/",
            "path": "/5fd80e0300e4c6ee00a1ade2",
            "httpMethod": "PUT",
            "queryStringParameters": "",
            "pathParameters": json!({
                "user_id": "5fd80e0300e4c6ee00a1ade2"
            }),
            "body": json!({
                "extra": json!({
                    "something": "algo"
                })
            })
        });
        match edit(event.clone(), Context::default()).await {
            Ok(_) => assert!(true),
            Err(_) => assert!(false)
        }
    }
}