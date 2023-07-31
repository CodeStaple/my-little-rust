use actix_web::{web, App, HttpResponse, HttpServer, Responder, post};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;
use serde::Deserialize;

use crate::models::{NewUser, User}; // assuming models.rs is in the same directory

//...

#[derive(Deserialize)]
struct UserInfo {
    name: String,
    email: String,
}

#[post("/users")]
async fn create_user(user_info: web::Json<UserInfo>, db_pool: web::Data<DbPool>) -> impl Responder {
    let new_user = NewUser {
        name: user_info.name.clone(),
        email: user_info.email.clone(),
    };
    let conn = db_pool.get().expect("Failed to get DB connection");

    HttpResponse::Ok().json(new_user) // Return the new user as JSON
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(hello)
            .service(create_user)
    );
}

//...
