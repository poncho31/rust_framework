// Global imports
use crate::models::_models::{Event, NewEvent}; // `crate` se réfère à la racine du projet (src)
use crate::schema::_schema::{events};          // `crate` se réfère à la racine du projet (src)
use crate::schema::_schema::events::dsl::*;    // Pour le DSL des tables Diesel
use crate::repository::_event_repository;

use tera::Tera;
use actix_web::{get, post, web, HttpResponse};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use serde::Deserialize;
use log::{info, warn, debug};

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;


// EVENTS LIST - Liste des événements
#[get("/")]
pub async fn list_events(pool: web::Data<DbPool>, tmpl: web::Data<Tera>) -> HttpResponse {

    let all_events = _event_repository::paginate_events(pool, None, None);

    debug!("all_events: {:#?}", all_events);

    let mut context = tera::Context::new();
    context.insert("events", &all_events);  // Insertion des événements dans le contexte

    let rendered = tmpl.render("index.html", &context).expect("Error rendering template");
    HttpResponse::Ok().body(rendered)  // Retour du rendu du template
}

// EVENTS ADD - Ajout d'un événement
#[post("/add_event")]
pub async fn add_event(event_data: web::Form<NewEventData>, pool: web::Data<DbPool>) -> HttpResponse {
    debug!("Début de la fonction add_event...");

    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(e) => {
            warn!("Impossible d'obtenir une connexion à la base de données : {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let new_event = NewEvent {
        title       : &event_data.title,
        description : event_data.description.as_deref(),
        date        : NaiveDateTime::parse_from_str(&event_data.date, "%Y-%m-%d %H:%M:%S").unwrap(),
        user_id     : event_data.user_id,
    };

    match _event_repository::insert_event(&new_event, &mut conn) {
        Ok(_) => {
            info!("Événement ajouté avec succès.");
            let html_data = format!(
                r#"
                <li class="box">
                    <h3 class="title is-4">{}</h3>
                    <p><strong>Date :</strong> {}</p>
                    <p><strong>Description :</strong> {}</p>
                </li>
                "#,
                event_data.title,
                event_data.date,
                event_data.description.as_deref().unwrap_or("Aucune description")
            );

            HttpResponse::Ok().json(serde_json::json!({
                "status" : "success",
                "message": "Événement ajouté avec succès.",
                "data"   : html_data
            }))
        },
        Err(e) => {
            warn!("Erreur lors de l'ajout de l'événement : {:?}", e);
            HttpResponse::InternalServerError().json("Failed to add event")
        }
    }
}


// FormData pour la soumission d'événements
#[derive(Deserialize)]
struct NewEventData {
    title: String,
    description: Option<String>,
    date: String,  // Format de date en chaîne de caractères
    user_id: i32,
}