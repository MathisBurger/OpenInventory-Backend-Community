use actix_web::{HttpRequest, Responder, web};
use crate::models::MessageModel;
use crate::models::LoginCredentials;
use crate::utils::MySQL;
use crate::utils::token;
use actix_web::Result;
use serde::{Deserialize, Serialize};
use actix_web::web::Json;
use std::borrow::Borrow;


pub async fn response(info: Json<LoginCredentials::LoginRequestModel>) -> Result<Json<MessageModel::MessageModel>> {
    let status = MySQL::login(&info.username, &info.password).await;
    if status {
        let token = token::generate();
        MySQL::updateToken(&info.username, &token).await;

        Ok(Json(MessageModel::MessageModel {
            message: "Login successful".to_string(),
            alert: "alert alert-success".to_string(),
            status: "ok".to_string(),
            token: token,
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