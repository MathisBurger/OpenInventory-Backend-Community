use crate::utils;
use sqlx::{mysql, Pool, MySql};
use crate::models::SQl_Structs;

pub async fn getConn() -> std::io::Result<Pool<MySql>> {
    let username = utils::loadFromConfig::loadParam(format!("{}", "database;username")).await.expect("Error while loading param from config");
    let password = utils::loadFromConfig::loadParam(format!("{}", "database;password")).await.expect("Error while loading param from config");
    let database = utils::loadFromConfig::loadParam(format!("{}", "database;database")).await.expect("Error while loading param from config");
    let host = utils::loadFromConfig::loadParam(format!("{}", "database;host")).await.expect("Error while loading param from config");
    Ok(mysql::MySqlPool::connect(&*format!("mysql://{}:{}@{}/{}", username, password, host, database)).await.expect("Error while creating connection"))
}

pub async fn login(displayname: &String, password: &String) -> bool {
    let mut conn = getConn().await.expect("Cannot connect to database");
    println!("Username: {}", displayname);
    let user = sqlx::query(format!("SELECT * FROM inv_users WHERE displayname='{}' AND password='{}'", displayname, password).as_str())
        .fetch_one(&conn).await;
    match user {
        Ok(user) => true,
        Err(err) => false,
    }
}