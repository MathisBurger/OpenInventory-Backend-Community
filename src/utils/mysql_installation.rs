use sqlx;
use crate::Var;
use std::borrow::BorrowMut;
use crate::utils::hashing;
use crate::utils::token;
use crate::models::SQl_Structs;
use sqlx::{MySql, Row};
use std::collections::HashMap;

pub async fn checkForTables(mut conn: &sqlx::Pool<MySql>) -> std::io::Result<()> {
    let tables = sqlx::query("SHOW TABLES LIKE 'inv_users';").fetch_all(conn).await.expect("Cannot get from database");
    if tables.len() == 1 {
        println!("Table: inv_users exists");
        Ok(())
    } else {
        generate_table(&conn, "inv_users".to_string()).await;
        Ok(())
    }
}

async fn generate_table(mut conn: &sqlx::Pool<MySql>, name: String) -> std::io::Result<()>{
    if name == "inv_users".to_string() {
        sqlx::query(Var::create_users_table_string().as_str()).execute(conn).await.expect("Cannot create Table");
        println!("Created database 'inv_users'");
        insert_default_user(&conn).await;
    }
    Ok(())
}

async fn insert_default_user(mut conn: &sqlx::Pool<MySql>) -> std::io::Result<()>{
    let hash = hashing::hash_including_salt("Admin123".to_string());
    let query = format!("INSERT INTO inv_users (id, username, password, token, root, mail, displayname, register_date, status) VALUES (NULL, 'root', '{}', 'None', '1', 'example@mail.de', 'root', current_timestamp(), 'enabled');", hash);
    sqlx::query(query.as_str()).execute(conn).await.expect("Cannot insert defult user");
    Ok(())
}

