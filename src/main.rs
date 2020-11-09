use actix_web::{App, HttpServer, web};
mod controller;
mod models;
mod utils;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    utils::installation::InstallationProcess().await;
    println!("Started Webserver on http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(controller::DefaultController::response))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}