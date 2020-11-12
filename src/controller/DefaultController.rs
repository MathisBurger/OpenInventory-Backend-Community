use actix_web::{Responder, web};
use crate::models::MessageModel::MessageModel;

pub async fn response() -> impl Responder {
    web::HttpResponse::Ok()
        .json(MessageModel {
            message: "API online".to_string(), alert: "alert alert-success".to_string(), status: "ok".to_string(), token: "token".to_string(), httpStatus: 200 }
        )
}