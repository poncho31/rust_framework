use crate::models::_models::{NewUser, NewUserData, User};
use crate::schema::_schema::{users};
use crate::schema::_schema::users::dsl::*;
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

use actix_web::{get, post, web, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use serde::Deserialize;
use bcrypt::{hash};
use log::{debug, info, warn};
use tera::Tera;
use crate::database::get_connection;
use crate::repository::{ _user_repository};
use crate::utils::ajax_message::add_user_message;

#[get("/users")]
pub async fn list_users(pool: web::Data<crate::database::DbPool>, tmpl: web::Data<Tera>) -> HttpResponse {

    let all_users = _user_repository::paginate_users(pool, None, None);

    let mut context = tera::Context::new();
    context.insert("users", &all_users);  // Insertion des événements dans le contexte

    let rendered = tmpl.render("user/user_manager.html", &context).expect("Error rendering template");
    HttpResponse::Ok().body(rendered)  // Retour du rendu du template
}

#[post("/add_user")]
pub async fn add_user(user_data: web::Form<NewUserData>, pool: web::Data<crate::database::DbPool>, tmpl: web::Data<Tera>) -> HttpResponse {
    debug!("Début de la fonction add_user...");

    let mut conn = match get_connection(pool) {
        Ok(conn) => conn,
        Err(err_response) => return err_response,  // En cas d'échec, retourner l'erreur HTTP
    };

    let new_user = &user_data.to_new_user();

    match _user_repository::insert_user(&new_user, &mut conn) {
        Ok(_) => {
            info!("Utilisateur ajouté avec succès.");
            add_user_message(user_data, tmpl)
        },
        Err(e) => {
            warn!("Erreur lors de l'ajout de l'utilisateur : {:?}", e);
            HttpResponse::InternalServerError().json("Failed to add user")
        }
    }
}


#[post("/login")]
pub async fn login(user_data: web::Json<LoginData>, pool: web::Data<DbPool>) -> HttpResponse {
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
