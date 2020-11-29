use crate::utils::MySQL::{login_with_token, getConn};
use crate::models::TableModel::TableModel;
use sqlx::Row;

pub async fn getAllTables(displayname: &String, password: &String, token: &String) -> Vec<TableModel> {
    let status = login_with_token(displayname, password, token).await;
    if !status {
        vec![]
    } else {
        let conn = getConn().await.expect("Cannot connect to database");
        let tables = sqlx::query("SELECT * FROM inv_tables")
            .fetch_all(&conn).await.expect("Cant get any tables");
        let mut names: Vec<TableModel> = vec![];
        if tables.is_empty() {
            conn.close();
            vec![]
        } else {
            for table in tables {
                let id: u32 = table.get(0);
                let name: String = table.get(1);
                let entries: i32 = table.get(2);
                let created_at: String = table.get(3);
                names.push(TableModel {id, name, entries, created_at});
            }
            conn.close();
            names
        }

    }
}