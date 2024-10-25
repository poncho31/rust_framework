use crate::models::_models::{NewUserData};
use crate::database::get_connection;
use crate::repository::{ _user_repository};
use crate::utils::ajax_message::add_user_message;

use actix_web::{get, post, web, HttpResponse};
use log::{debug, info, warn};
use tera::Tera;

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