use actix_web::{App, HttpServer, web, http};
mod controller;
mod models;
mod utils;
mod Var;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    utils::installation::InstallationProcess().await;
    println!("Started Webserver on http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(controller::DefaultController::response))
            .route("/login", web::post().to(controller::LoginController::response))

    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}