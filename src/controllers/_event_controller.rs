// Global imports
use crate::models::_models::{Event, NewEvent}; // `crate` se réfère à la racine du projet (src)
use crate::schema::_schema::{events};          // `crate` se réfère à la racine du projet (src)
use crate::schema::_schema::events::dsl::*;    // Pour le DSL des tables Diesel

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
    let mut conn = pool.get().expect("Couldn't get DB connection");

    let all_events = events.load::<Event>(&mut conn).expect("Error loading events");

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

    match diesel::insert_into(events::table)
        .values(&new_event)
        .execute(&mut conn)
    {
        Ok(_) => {
            info!("Événement ajouté avec succès.");
            // HttpResponse::Found().append_header(("Location", "/")).finish()
            // Générer le HTML avec les données de l'événement
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

            // Renvoyer la réponse avec le HTML généré
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

// EVENTS FORM - Formulaire d'ajout d'événement
#[get("/add_event")]
pub async fn show_add_event_form(tmpl: web::Data<Tera>) -> HttpResponse {
    let rendered = tmpl.render("modal_add_event.html", &tera::Context::new()).expect("Error rendering template");
    HttpResponse::Ok().body(rendered)
}
