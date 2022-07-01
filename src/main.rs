#![allow(unused_variables)]


use actix_web::{web, App, HttpServer};
use mongodb::{Client};

#[path = "routes/index.rs"]
mod routes;
#[path = "app/constants/index.rs"]
mod constants;

pub fn init(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(web::scope("/routes"))
            .service(routes::index)
            .service(routes::create_user)
            .service(routes:: get_user)
            .service(routes:: get_all_users)
            .service(routes:: delete_user)
            .service(routes:: update_user)
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Client::with_uri_str(constants::DB_URL).await.expect("failed to connect");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .configure(init)
    })
        .bind(("127.0.0.1", 3001))?
        .run()
        .await
}
