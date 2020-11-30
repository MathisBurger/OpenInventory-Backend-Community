use actix_web::web::Json;
use actix_web::Result;
use crate::models::{LoginCredentials, MessageModel};
use crate::utils::table_sql::AllTables::getAllTables;

pub async fn response(info: Json<LoginCredentials::LoginRequestModel>) -> Result<Json<MessageModel::MessageModel>> {
    let tables = getAllTables(&info.username, &info.password, &info.token).await;
    let mut compiled_tables: Vec<String> = vec![];
    for table in tables {
        compiled_tables.push(format!("['{}','{}','{}']", table.name, table.entries, table.created_at));
    }
    let mut msg = String::new();
    for i in 0..compiled_tables.len() {
        msg += format!("{};", compiled_tables[i]).as_str();
    }
    Ok(Json(MessageModel::MessageModel {
        message: msg,
        alert: "alert alert-success".to_string(),
        status: "ok".to_string(),
        token: "None".to_string(),
        httpStatus: 200
    }))
}