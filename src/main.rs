/*
 * Copyright (c) 2023 - Chatter Social, Inc.
 * Created by: Justin B. Watson (Geeken)
 */

use serde::Deserialize;
use std::fmt::Debug;
use surrealdb::engine::remote::ws::Wss;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Deserialize)]
struct User {
    #[allow(dead_code)]
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
    println!("{:?}", user.id.to_raw());

    Ok(())
}
