use mysql::*;
use mysql::prelude::*;
struct table {
    name: String
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

pub fn checkForTables(mut conn: &PooledConn) -> bool {
    let tables = conn.query_map("SHOW TABLES;", |table_name|{ table {name: table_name}}).expect("Cannot get all tables");
    if checkIfContain(tables, "inv_users".to_string()) {
        true
    } else {
        false
    }
}

pub fn generateTable(mut conn: &PooledConn, name: String) -> bool {
    true
}

