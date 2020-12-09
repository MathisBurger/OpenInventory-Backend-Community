use actix_web::web::Json;
use crate::models::{LoginCredentials, MessageModel};
use crate::utils::table_sql::CreateTable;
use actix_web::Result;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct json_model {
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) token: String,
    pub(crate) table_name: String,
    pub(crate) row_config: String
}

pub async fn response(info: Json<json_model>) -> Result<Json<MessageModel::MessageModel>> {
    let status = CreateTable::create_table(
        &info.username,
        &info.password,
        &info.token,
        &check_table_name(&info.table_name),
        &parse(&info.row_config)
    ).await;
    if status {
        Ok(Json(MessageModel::MessageModel {
            message: "successful".to_string(),
            alert: "alert alert-success".to_string(),
            status: "ok".to_string(),
            token: "None".to_string(),
            httpStatus: 200
        }))
    } else {
        Ok(Json(MessageModel::MessageModel {
            message: "creation failed".to_string(),
            alert: "alert alert-danger".to_string(),
            status: "ok".to_string(),
            token: "None".to_string(),
            httpStatus: 200
        }))
    }
}

fn parse(val: &String) -> Vec<(String, String)> {
    let arr = val.split(",").collect::<Vec<&str>>();
    let mut tuples: Vec<(String, String)> = vec![];
    for el in arr {
        let raws = el
            .replace("(", "")
            .replace(")", "")
            .replace(" ", "")
            .replace("[", "")
            .replace("]", "")
            .replace("-", "_");
        let spl = raws.split(";").collect::<Vec<&str>>();
        tuples.push((spl[0].to_string(), spl[1].to_string()));
    }
    tuples
}

fn check_table_name(name: &String) -> String {
    name.replace("-", "_")
}