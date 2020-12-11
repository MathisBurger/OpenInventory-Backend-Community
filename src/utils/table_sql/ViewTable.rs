use crate::utils::MySQL::{getConn, login_with_token};
use sqlx::mysql::{MySqlRow, MySqlColumn};
use sqlx::{Error, Row, Column};
use actix_cors::AllOrSome;

pub async fn view_table(displayname: &String, password: &String, token: &String, table_name: &String, operation: &String) -> String {
    let status = login_with_token(displayname, password, token).await;
    if !status {
        "None".to_string()
    } else {
        let conn = getConn().await.expect("Cannot connect to database");
        if operation == "None" {
            let sql = format!("SELECT * FROM table_{}", table_name);
            let entrys_raw = sqlx::query(sql.as_str()).fetch_all(&conn).await;
            match entrys_raw {
                Ok(_) => {
                    let mut entrys = vec![];
                    for x in entrys_raw.unwrap() {
                        let mut cache = String::new().to_owned();
                        for i in 0..x.len() {
                            match x.columns().get(0).unwrap().type_info().to_string().as_str() {
                                "INT UNSIGNED" => {
                                    let p: i32 = x.get(i);
                                    cache += format!("{}", p).as_str();
                                }
                                _ => {println!("Kritisch");}
                            };
                            entrys.push(&cache);
                        }

                    }
                    println!("{:?}", entrys);
                    "All".to_string()
                }
                Err(_) => {"None".to_string()}
            }
        } else {
            "None".to_string()
        }
    }
}
