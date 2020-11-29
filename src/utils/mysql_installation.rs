use sqlx;
use crate::Var;
use std::borrow::{BorrowMut, Borrow};
use crate::utils::hashing;
use crate::utils::token;
use crate::models::SQl_Structs;
use sqlx::{MySql, Row};
use std::collections::HashMap;

pub async fn checkForTables(mut conn: &sqlx::Pool<MySql>) -> std::io::Result<()> {
    let tables = sqlx::query("SHOW TABLES LIKE 'inv_%';").fetch_all(conn).await.expect("Cannot get from database");
    if tables.len() == 2 {
        println!("All required tables are existing");
        Ok(())
    } else {
        let required_tables = vec![
            "inv_users".to_string(),
            "inv_tables".to_string()
        ];
        let mut active_tables = vec![];
        for i in 0..tables.len() {
            let table: String = tables[i].get(0);
            active_tables.push(table);
        }
        let mut outstanding_tables = vec![];
        for i in 0..required_tables.len() {
            if !active_tables.contains(&required_tables[i]) {
                outstanding_tables.push(String::from(&required_tables[i]));
            } else {
                println!("Table {} exists", &required_tables[i]);
            }
        }
        for i in 0..outstanding_tables.len() {
            generate_table(&conn, String::from(&outstanding_tables[i])).await;
        }
        Ok(())
    }
}

async fn generate_table(mut conn: &sqlx::Pool<MySql>, name: String) -> std::io::Result<()>{
    if name == "inv_users".to_string() {
        sqlx::query(Var::create_users_table_string().as_str()).execute(conn).await.expect("Cannot create Table");
        insert_default_user(&conn).await;
    } else if name == "inv_tables".to_string() {
        sqlx::query(Var::create_tables_table_string().as_str()).execute(conn).await.expect("Cannot create Table");
    }
    println!("Created table {}", name);
    Ok(())
}

async fn insert_default_user(mut conn: &sqlx::Pool<MySql>) -> std::io::Result<()>{
    let hash = hashing::hash_including_salt("Admin123".to_string());
    let query = format!("INSERT INTO inv_users (id, username, password, token, root, mail, displayname, register_date, status) VALUES (NULL, 'root', '{}', 'None', '1', 'example@mail.de', 'root', current_timestamp(), 'enabled');", hash);
    sqlx::query(query.as_str()).execute(conn).await.expect("Cannot insert defult user");
    Ok(())
}

