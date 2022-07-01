#![allow(unused_variables)]


use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use mongodb::Client;

#[path = "../app/modules/user/index.rs"]
mod user_controller;

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Rust microservice alive!")
}

#[get("/user")]
pub async fn get_user(client: web::Data<Client>) -> impl Responder {
    let user_details = user_controller::get_user(client).await;
    HttpResponse::Ok().json("User details !")
}

#[post("/create-user")]
pub async fn create_user(client: web::Data<Client>) -> impl Responder {
    let user_details = user_controller::create_user(client).await;
    HttpResponse::Ok().body("User Created !")
}

#[get("/get-all-users")]
pub async fn get_all_users(client: web::Data<Client>) -> HttpResponse {
    let user_details = user_controller::get_all_users(client).await;
    HttpResponse::Ok().json("All user details !")
}

#[put("/update-user")]
pub async fn update_user(client: web::Data<Client>) -> impl Responder {
    let user_details = user_controller::update_user(client).await;
    HttpResponse::Ok().body("update user api........")
}

#[delete("/delete-user")]
pub async fn delete_user(client: web::Data<Client>) -> impl Responder {
    let user_details = user_controller::delete_user(client).await;
    HttpResponse::Ok().body("Delete User Success !")
}
