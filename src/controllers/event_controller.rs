use crate::models::{Event, NewEvent};  // Import des modèles Event et NewEvent
use crate::schema::{users, events};    // Import des schémas des tables users et events

use tera::{Tera, Context};
use actix_web::{get, post, web, HttpResponse};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use serde::Deserialize;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

// EVENTS LIST - Liste des événements
#[get("/")]
pub async fn list_events(pool: web::Data<DbPool>, tmpl: web::Data<Tera>) -> HttpResponse {
    use crate::schema::events::dsl::*;  // Utilisation de la table events depuis le schéma

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
    use crate::schema::events;  // Import de la table events

    let mut conn = pool.get().expect("Couldn't get DB connection");

    let new_event = NewEvent {
        title: &event_data.title,
        description: event_data.description.as_deref(),
        date: NaiveDateTime::parse_from_str(&event_data.date, "%Y-%m-%d %H:%M:%S").unwrap(),
        user_id: event_data.user_id,
    };

    match diesel::insert_into(events::table)
        .values(&new_event)
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Ok().json("Event added successfully"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to add event"),
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
    let rendered = tmpl.render("add_event.html", &tera::Context::new()).expect("Error rendering template");
    HttpResponse::Ok().body(rendered)
}
