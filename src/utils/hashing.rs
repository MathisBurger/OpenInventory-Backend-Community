use pwhash::sha512_crypt;
use crate::utils::MySQL;
use sqlx::Row;

pub fn hash_including_salt(pwd: String) -> String {
    sha512_crypt::hash(pwd).expect("Could not hash password")
}

pub async fn hash_from_database(displayname: &String, password: &String) -> std::io::Result<String>{
    let mut conn = MySQL::getConn().await.expect("Cannot connect to database");

    let user = sqlx::query(format!("SELECT * FROM inv_users WHERE displayname='{}'", displayname).as_str())
        .fetch_one(&conn).await;
    let contains = match &user {
        Ok(row) => true,
        Err(err) => false,
    };
    if contains {
        let pwd: String = user.unwrap().get("password");
        Ok(sha512_crypt::hash_with(pwd.as_str(), password.as_str()).unwrap())
    } else {
        Ok("nothing".to_string())
    }

}