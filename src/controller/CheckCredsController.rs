use actix_web::web::Json;
use crate::models::{LoginCredentials, MessageModel};
use crate::utils::MySQL;
use actix_web::Result;

pub async fn response(info: Json<LoginCredentials::LoginRequestModel>) -> Result<Json<MessageModel::MessageModel>> {
    let status = MySQL::login_with_token(&info.username, &info.password, &info.token).await;
    if status {
        Ok(Json(MessageModel::MessageModel {
            message: "Login successful".to_string(),
            alert: "alert alert-success".to_string(),
            status: "ok".to_string(),
            token: "None".to_string(),
            httpStatus: 200
        }))
    } else {
        Ok(Json(MessageModel::MessageModel {
            message: "Login failed".to_string(),
            alert: "alert alert-danger".to_string(),
            status: "ok".to_string(),
            token: "None".to_string(),
            httpStatus: 200
        }))
    }

}