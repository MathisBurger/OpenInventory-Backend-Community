use actix_web::{App, HttpServer, web, http};
use actix_cors::Cors;
use crate::utils::table_sql::ViewTable::view_table;

mod controller;
mod models;
mod utils;
mod Var;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    utils::installation::InstallationProcess().await;
    println!("Started Webserver on http://0.0.0.0:8080");
    view_table(&"root".to_string(), &"Admin123".to_string(), &"CGaXR8k0rgbVIKQCYinFBAeO0XSKu9Cq".to_string(),
               &"iq_tabelle".to_string(), &"None".to_string()).await;
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::new().supports_credentials()
                .finish()
            )
            .route("/", web::get().to(controller::DefaultController::response))
            .route("/login", web::post().to(controller::LoginController::response))
            .route("/check_creds", web::post().to(controller::CheckCredsController::response))
            .route("/table_management/get_all_tables", web::post().to(controller::AllTablesController::response))
            .route("/table_management/create_table", web::post().to(controller::CreateTableController::response))

    })
        .bind("0.0.0.0:8080")?
        .run()
        .await

}