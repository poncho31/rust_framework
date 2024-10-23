// Global imports
use crate::models::_models::{NewEventData};
use crate::repository::_event_repository;
use crate::database::{get_connection, DbPool};

use tera::{ Context, Tera};
use actix_web::{get, post, web, HttpResponse};
use chrono::NaiveDateTime;
use log::{info, warn, debug};


// EVENTS LIST - Liste des événements
#[get("/")]
pub async fn list_events(pool: web::Data<DbPool>, tmpl: web::Data<Tera>) -> HttpResponse {

    let all_events = _event_repository::paginate_events(pool, None, None);
    // debug!("all_events: {:#?}", all_events);

    let mut context = tera::Context::new();
    context.insert("events", &all_events);  // Insertion des événements dans le contexte

    let rendered = tmpl.render("index.html", &context).expect("Error rendering template");
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

            // Contexte pour un seul événement
            let mut context = tera::Context::new();
            context.insert("title", &event_data.title);
            context.insert("date", &event_data.date);
            context.insert("description", &event_data.description.as_deref().unwrap_or("Aucune description"));

            // Rendre la macro event_item pour cet événement
            let html_data = tmpl.render("html_module/shared/event_item_module_ajax.html", &context)
                .expect("Erreur lors du rendu du template");

            HttpResponse::Ok().json(serde_json::json!({
                "status"        : "success",
                "message"       : "Événement ajouté avec succès.",
                "html_response" : html_data,
                "data"          : event_data
            }))
        },
        Err(e) => {
            warn!("Erreur lors de l'ajout de l'événement : {:?}", e);
            HttpResponse::InternalServerError().json("Failed to add event")
        }
    }
}