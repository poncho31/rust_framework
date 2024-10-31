// Global imports
use crate::models::_models::{NewEventData};
use crate::repository::_event_repository;
use crate::database::{get_connection, DbPool};
use crate::utils::ajax_message::{add_event_message};
// use crate::view::event_manager_builder::{EventManager, EventTable};
// use crate::view::event_table::EventItem;
use tera::Tera;
use actix_web::{get, post, web, HttpResponse};
use log::{info, warn, debug};

use crate::utils::builder::page::module::nav_bar::NavBar;


// EVENTS LIST - Liste des événements
#[get("/")]
pub async fn list_events(pool: web::Data<DbPool>, tmpl: web::Data<Tera>) -> HttpResponse {

    let all_events = _event_repository::paginate_events(pool, None, None);

    let mut context = tera::Context::new();
    context.insert("events", &all_events);  // Insertion des événements dans le contexte

    let rendered = tmpl.render("event/event_manager.html", &context).expect("Error rendering template");
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

// #[get("/event_manager")]
// pub async fn test_event_manager(pool: web::Data<DbPool>) -> HttpResponse {
//     // Récupération des événements depuis la base de données
//     let db_events = _event_repository::paginate_events(pool, None, None);
//
//     // Transformation des données de la base en `EventItem` pour EventTable
//     let event_items: Vec<EventItem> = db_events.iter().map(|event| {
//         EventItem {
//             id: event.id.expect("ID manquant"),
//             title: event.title.clone(),
//             description: event.description.clone(),
//             date: event.date.to_string(),
//         }
//     }).collect();
//
//     // Création de la barre de navigation et de la table des événements
//     let navbar = NavBar::new("MainNav".to_string(), Some("Événements".to_string()), None);
//     let event_table = EventTable::new(event_items);
//
//     // Utilisation d'EventManager pour générer le HTML
//     let event_manager = EventManager::new(navbar, event_table);
//     let html_output = event_manager.render_page();
//
//     HttpResponse::Ok().content_type("text/html").body(html_output)
// }