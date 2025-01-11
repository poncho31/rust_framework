// Global imports
use crate::repository::event_repository;
use crate::database::{get_connection, DbPool};
use crate::utils::ajax_message::{add_event_message};
use tera::Tera;
use actix_web::{web, HttpResponse};
use log::{info, warn, debug};
use crate::models::event_model::NewEventData;
use crate::utils::builder::page_builder::list::List;
use crate::utils::builder::page_builder::page_builder::PageBuilder;
use crate::utils::builder::page_builder::section::DataType;
use crate::utils::builder::page_builder::table::Table;
use crate::utils::template_engine::template::generate_html;


// EVENTS LIST - Liste des événements
pub async fn list_events(pool: web::Data<DbPool>, tmpl: web::Data<Tera>) -> HttpResponse {
    /// Récupération des données des événements
    let all_events = event_repository::paginate_events(pool, None, Some(100));

    /// Construction de l'objet PageBuilder
    let page_builder = PageBuilder::base_model(
        /// NAVBAR
        "Rust framework",
        "Page title",
        Some(vec![
            ("Homepage".to_string(), "/".to_string()),
            ("Utilisateurs".to_string(), "/users".to_string()),
            ("Déconnexion".to_string(), "/users/logout".to_string()),
            ("Page builder".to_string(), "/page/builder".to_string()),
        ]),
        Some(vec![
            ("Utilisateurs".to_string(), "/users".to_string()),
            ("Déconnexion".to_string(), "/users/logout".to_string()),
        ]),
        /// SECTION
        "Welcome Section",
        vec![
            DataType::Table(Table::from("event_table", all_events.clone())),
            DataType::List(List::from("event_list", all_events.clone()))
        ], // Injecte le tableau dans la section
    );

    /// Génération de l'html avec injection des données
    let html_output = generate_html("tera", page_builder);

    /// Retourner le HTML généré dans la réponse HTTP
    HttpResponse::Ok().content_type("text/html").body(html_output)
}


// EVENTS ADD - Ajout d'un événement
pub async fn add_event(event: web::Form<NewEventData>, pool: web::Data<DbPool>, tmpl: web::Data<Tera>) -> HttpResponse {
    debug!("Début de la fonction add_event...");

    let mut conn = match get_connection(pool) {
        Ok(conn) => conn,
        Err(err_response) => return err_response,  // En cas d'échec, retourner l'erreur HTTP
    };

    match event_repository::insert_event(&event.new(), &mut conn) {
        Ok(_) => {
            info!("Événement ajouté avec succès.");
            add_event_message(event, tmpl)
        },
        Err(e) => {
            warn!("Erreur lors de l'ajout de l'événement : {:?}", e);
            HttpResponse::InternalServerError().json("Failed to add event")
        }
    }
}
