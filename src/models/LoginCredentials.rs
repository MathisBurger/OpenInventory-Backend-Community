use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequestModel {
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) token: String
}