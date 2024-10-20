use crate::models::{NewUser, User};
use tera::{Tera, Context};

use actix_web::{get ,post, web, HttpResponse};
use chrono::NaiveDateTime;
use crate::models::NewEvent;
use crate::DbPool; // Assure-toi que DbPool est bien défini dans ton projet
use diesel::prelude::*;

use crate::schema::event;

use serde::Deserialize;
mod models;
mod schema;

#[post("/register")]
pub async fn register(user_data: web::Json<RegisterData>, pool: web::Data<DbPool>) -> HttpResponse {
    use schema::users;

    let mut conn = pool.get().expect("Couldn't get DB connection");

    let new_user = NewUser {
        username: &user_data.username,
        email: &user_data.email,
        password_hash: &hash(&user_data.password, 4).unwrap(),
    };

    match diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn)  // Ajout de `mut` ici
    {
        Ok(_) => HttpResponse::Ok().json("User registered successfully"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to register user"),
    }
}

#[derive(Deserialize)]
struct RegisterData {
    username: String,
    email: String,
    password: String,
}

#[post("/login")]
pub async fn login(user_data: web::Json<LoginData>, pool: web::Data<DbPool>) -> HttpResponse {
    use schema::users::dsl::{users, email, password_hash};

    let mut conn = pool.get().expect("Couldn't get DB connection");

    // Rechercher l'utilisateur par email
    match users.filter(email.eq(&user_data.email)).first::<User>(&mut conn) {
        Ok(user) => {
            // Vérifier le mot de passe
            if bcrypt::verify(&user_data.password, &user.password_hash).unwrap() {
                HttpResponse::Ok().json("Login successful")
            } else {
                HttpResponse::Unauthorized().json("Invalid credentials")
            }
        }
        Err(_) => HttpResponse::Unauthorized().json("Invalid credentials"),
    }
}

#[derive(Deserialize)]
struct LoginData {
    email: String,
    password: String,
}
