use actix_web::{HttpRequest, Responder, web};
use crate::models::MessageModel;
use crate::models::LoginCredentials;
use crate::utils::MySQL;
use actix_web::Result;
use serde::{Deserialize, Serialize};
use actix_web::web::Json;
use std::borrow::Borrow;


pub async fn response(info: Json<LoginCredentials::LoginRequestModel>) -> Result<Json<MessageModel::MessageModel>> {
    let status = MySQL::login(&info.username, &info.password).await;
    Ok(Json(MessageModel::MessageModel {
        message: "".to_string(),
        alert: "".to_string(),
        status: "".to_string(),
        token: "".to_string(),
        httpStatus: 0
    }))
}