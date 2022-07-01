#![allow(unused_variables)]


use actix_web::{web, web::Json, HttpResponse, Responder};
use mongodb::bson::doc;
use mongodb::{Client, Collection};

#[path = "../../constants/index.rs"]
mod constants;
#[path = "../../models/user.rs"]
mod model;

use model::User;

// Create User
pub async fn create_user(client: web::Data<Client>) -> impl Responder {
    let collection: Collection<User> = client
        .database(constants::DB_NAME)
        .collection(constants::USER_COLLECTION);

    let doc = User {
        first_name: String::from("D"),
        last_name: String::from("K"),
        username: String::from("kkamlesh1234"),
        email: String::from("kkamlesh1234@leewayhertz.com"),
    };

    let res = collection.insert_one(doc, None).await;
    match collection
        .find_one(doc! { "email": "kamlesh@leewayhertz.com" }, None)
        .await
    {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body(format!("No user found with username")),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// Get User
pub async fn get_user(client: web::Data<Client>) -> mongodb::error::Result<Option<User>> {
    let collection: Collection<User> = client
        .database(constants::DB_NAME)
        .collection(constants::USER_COLLECTION);
    let user_data = collection
        .find_one(doc! { "email": "kamlesh@leewayhertz.com" }, None)
        .await;

    println!("Users ----------------------------> {:?}", user_data);
    return user_data;
}

// Get All User
pub async fn get_all_users(client: web::Data<Client>) {
    let collection: Collection<User> = client
        .database(constants::DB_NAME)
        .collection(constants::USER_COLLECTION);
    let users = collection.find(doc! {}, None).await;

    println!("users------------------> {:?}", Json(users));
}

// Update User
pub async fn update_user(client: web::Data<Client>) -> mongodb::error::Result<Option<User>> {
    let collection: Collection<User> = client
        .database(constants::DB_NAME)
        .collection(constants::USER_COLLECTION);
    let data = collection
        .find_one_and_update(
            doc! {"email" : "kkamlesh1234@leewayhertz.com"},
            doc! {"$set" : {"first_name" : "Sam"}},
            None,
        )
        .await;
    println!("update data ... {:?}", data);
    return data;
}

// Delete User
pub async fn delete_user(client: web::Data<Client>) {
    let collection: Collection<User> = client
        .database(constants::DB_NAME)
        .collection(constants::USER_COLLECTION);
    let _delete_response = collection
        .find_one_and_delete(doc! {"email": "kkamlesh1234@leewayhertz.com"}, None)
        .await;
    let users = collection.find(doc! {}, None).await;
    println!("After Delete ------------------> {:?}", Json(users),);
}


