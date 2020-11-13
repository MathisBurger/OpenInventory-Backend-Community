use mysql::*;
use mysql::prelude::*;
use crate::Var;
use std::borrow::BorrowMut;
use crate::utils::hashing;
use crate::utils::token;
struct table {
    name: String
}

struct param {
    param: String
}

fn checkIfContain(tables: Vec<table>, name: String) -> bool {
    let mut contains: bool = false;
    for tabl in tables {
        if tabl.name == name {
            contains = true;
        }
    }
    contains
}

pub fn checkForTables(conn: &mut PooledConn) -> std::io::Result<()> {
    let tables = conn.query_map("SHOW TABLES;", |table_name|{ table {name: table_name}}).expect("Cannot get all tables");
    if checkIfContain(tables, "inv_users".to_string()) {
        println!("Table: inv_users exists");
        Ok(())
    } else {
        generate_table(conn.borrow_mut(), "inv_users".to_string());
        Ok(())
    }
}

fn generate_table(conn: &mut PooledConn, name: String) -> std::io::Result<()>{
    if name == "inv_users".to_string() {
        conn.query_drop(Var::create_users_table_string().as_str()).expect("Cannot create MySQL Table");
        println!("Created database 'inv_users'");
        insert_default_user(conn);
    }
    Ok(())
}

fn insert_default_user(conn: &mut PooledConn) -> std::io::Result<()>{
    let hash = hashing::hash_including_salt("Admin123".to_string());
    conn.exec_drop(format!("INSERT INTO inv_users (id, username, password, token, root, mail, displayname, register_date, status) VALUES (NULL, 'root', '{}', 'None', '1', 'example@mail.de', 'root', current_timestamp(), 'enabled');", hash),
                   ()
    ).expect("Cannot write default user to database");
    Ok(())
}

