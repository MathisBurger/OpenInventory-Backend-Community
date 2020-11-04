use serde::Serialize;

#[derive(Serialize)]
pub struct MessageModel {
    pub message: String,
    pub alert: String,
    pub status: String,
    pub token: String,
    pub httpStatus: i32
}