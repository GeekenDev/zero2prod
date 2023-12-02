/*
 * Copyright (c) 2023 - Chatter Social, Inc.
 * Created by: Justin B. Watson (Geeken)
 */
use serde::Deserialize;
use serde_json::json;
//use std::fmt::Debug;
use surrealdb::engine::remote::ws::Wss;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Deserialize)]
struct User {
    // #[allow(dead_code)]
    id: Thing,
    username: String,
    first_name: Option<String>,
    last_name: Option<String>,
    email: String,
    email_verified: Option<bool>,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // Connect to the server
    let db = Surreal::new::<Wss>("dev-db.chattersocial.io").await?;

    // Sign in as a namespace, database, or root user
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Select a specific namespace / database
    db.use_ns("dev").use_db("dev").await?;

    // Select all people records
    let people: Vec<User> = db.select("user").await?;

    let filtered_users: Vec<_> = people
        .iter()
        .filter(|user| user.email_verified == Some(true))
        .collect();
    let user = filtered_users[0];

    let obj = json!({
        "id": user.id.to_raw(),
        "username": user.username,
        "email": user.email,
        "email_verified": user.email_verified,
        "first_name": user.first_name,
        "last_name": user.last_name
    });

    println!("{}", serde_json::to_string_pretty(&obj).unwrap());

    Ok(())
}
