use actix_web::{HttpRequest, Responder, web};
use crate::models::{MessageModel, request};
use crate::utils::MySQL;
use serde_json;
use actix_web::web::Json;

pub async fn response(req: HttpRequest) -> impl Responder {

    web::HttpResponse::Ok()
        .json(MessageModel::MessageModel {
            message: "Kritischer Hase".to_string(),
            alert: "alert alert-success".to_string(),
            status: "ok".to_string(),
            token: "token".to_string(),
            httpStatus: 200
        })
}