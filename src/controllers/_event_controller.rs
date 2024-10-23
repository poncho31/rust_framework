// Global imports
use crate::models::_models::{NewEventData};
use crate::repository::_event_repository;
use crate::database::{get_connection, DbPool};
use crate::utils::ajax_message::{add_event_message};

use tera::Tera;
use actix_web::{get, post, web, HttpResponse};
use chrono::NaiveDateTime;
use log::{info, warn, debug};


// EVENTS LIST - Liste des événements
#[get("/")]
pub async fn list_events(pool: web::Data<DbPool>, tmpl: web::Data<Tera>) -> HttpResponse {

    let all_events = _event_repository::paginate_events(pool, None, None);

    let mut context = tera::Context::new();
    context.insert("events", &all_events);  // Insertion des événements dans le contexte

    let rendered = tmpl.render("event_manager.html", &context).expect("Error rendering template");
    HttpResponse::Ok().body(rendered)  // Retour du rendu du template
}


// EVENTS ADD - Ajout d'un événement
#[post("/add_event")]
pub async fn add_event(event_data: web::Form<NewEventData>, pool: web::Data<DbPool>, tmpl: web::Data<Tera>) -> HttpResponse {
    debug!("Début de la fonction add_event...");

    let mut conn = match get_connection(pool) {
        Ok(conn) => conn,
        Err(err_response) => return err_response,  // En cas d'échec, retourner l'erreur HTTP
    };

    let new_event = &event_data.to_new_event();

    match _event_repository::insert_event(&new_event, &mut conn) {
        Ok(_) => {
            info!("Événement ajouté avec succès.");
            add_event_message(event_data, tmpl)
        },
        Err(e) => {
            warn!("Erreur lors de l'ajout de l'événement : {:?}", e);
            HttpResponse::InternalServerError().json("Failed to add event")
        }
    }
}