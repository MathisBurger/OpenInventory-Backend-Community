use crate::utils;
use sqlx::{mysql, Pool, MySql};
use crate::models::SQl_Structs;
use std::borrow::Borrow;

pub async fn getConn() -> std::io::Result<Pool<MySql>> {
    let username = utils::loadFromConfig::loadParam(format!("{}", "database;username")).await.expect("Error while loading param from config");
    let password = utils::loadFromConfig::loadParam(format!("{}", "database;password")).await.expect("Error while loading param from config");
    let database = utils::loadFromConfig::loadParam(format!("{}", "database;database")).await.expect("Error while loading param from config");
    let host = utils::loadFromConfig::loadParam(format!("{}", "database;host")).await.expect("Error while loading param from config");
    Ok(mysql::MySqlPool::connect(&*format!("mysql://{}:{}@{}/{}", username, password, host, database)).await.expect("Error while creating connection"))
}

pub async fn login(displayname: &String, password: &String) -> bool {
    let mut conn = getConn().await.expect("Cannot connect to database");
    let hash = utils::hashing::hash_from_database(displayname, password)
        .await.expect("Cannot read hash");
    let user = sqlx::query(format!("SELECT * FROM inv_users WHERE displayname='{}' AND password='{}'", displayname, hash).as_str())
        .fetch_one(&conn).await;
    conn.close();
    match user {
        Ok(user) => true,
        Err(err) => false,
    }
}

pub async fn login_with_token(displayname: &String, password: &String, token: &String) -> bool {
    let mut conn = getConn().await.expect("Cannot connect to database");
    let hash = utils::hashing::hash_from_database(displayname, password)
        .await.expect("Cannot read hash");
    let user = sqlx::query(format!("SELECT * FROM inv_users WHERE displayname='{}' AND password='{}' AND token='{}'", displayname, hash, token).as_str())
        .fetch_one(&conn).await;
    conn.close();
    match user {
        Ok(user) => true,
        Err(err) => false,
    }
}

pub async fn updateToken(displayname: &String, token: &String) -> bool {
    let mut conn = getConn().await.expect("Cannot connect to database");
    let status = sqlx::query(format!("UPDATE inv_users SET token ='{}' WHERE displayname='{}';", token, displayname).as_str())
        .execute(&conn).await;
    conn.close();
    match status {
        Ok(o) => true,
        Err(err) => false,
    }
}

