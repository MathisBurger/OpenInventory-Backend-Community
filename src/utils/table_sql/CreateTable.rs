use crate::utils::MySQL::{login_with_token, getConn};
use crate::models::TableModel::TableModel;
use sqlx::{Row, Statement, Error};
use sqlx::mysql::MySqlDone;

pub async fn create_table(displayname: &String, password: &String, token: &String, table_name: &String, row_config: &Vec<(String, String)>) -> bool {
    let status = login_with_token(displayname, password, token).await;
    if !status {
        false
    } else {
        let mut cache = String::new();
        for row in row_config {

            cache = format!("{}{}", cache, check_type(&row));
        }
        let mut chars: Vec<char> = cache.chars().collect();
        let index = (chars.len() - 1);
        let mut fin_str = String::new();
        for i in 0..chars.len() {
            if i != index {
                fin_str = format!("{}{}", fin_str, chars[i]);
            }
        }
        let creation_string = format!("CREATE TABLE IF NOT EXISTS table_{}(id INT(6) UNSIGNED AUTO_INCREMENT PRIMARY KEY, {});", table_name, fin_str);

        let conn = getConn().await.expect("Cannot connect to database");
        let resp = sqlx::query(creation_string.as_str()).execute(&conn).await;
        match resp {
            Ok(_) => {
                let sql = format!("INSERT INTO inv_tables (id, name, entries, created_at) VALUES (NULL, 'table_{}', '0', current_timestamp()) WHERE NOT EXISTS (SELECT name FROM inv_tables WHERE name='table_{}');", table_name, table_name);
                sqlx::query(sql.as_str()).execute(&conn).await;
                conn.close();
                true
            }
            Err(_) => {
                conn.close();
                false
            }
        }
    }
}

fn check_type(config: &(String, String)) -> String {
    match config.0.as_str() {
        "INT" => format!("{} INT(11),", config.1),
        "FLOAT" => format!("{} float,", config.1),
        "BOOLEAN" => format!("{} tinyint(1)", config.1),
        "STRING8Chars" => format!("{} VARCHAR(8),", config.1),
        "STRING16Chars" => format!("{} VARCHAR(16),", config.1),
        "STRING64Chars" => format!("{} VARCHAR(64),", config.1),
        "STRING128Chars" => format!("{} VARCHAR(128),", config.1),
        "STRING1024Chars" => format!("{} VARCHAR(1024),", config.1),
        _ => {String::new()}
    }
}
