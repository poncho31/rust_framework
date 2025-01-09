use actix_web::{HttpResponse, web};
use crate::database::DbPool;
use crate::repository::event_repository;
use crate::utils::builder::page_builder::list::List;
use crate::utils::builder::page_builder::page_builder::PageBuilder;
use crate::utils::builder::page_builder::section::DataType;
use crate::utils::builder::page_builder::table::Table;
use crate::utils::template_engine::template::{generate_html};
use crate::view::event_table::{EventItem};

pub async fn test_inject_object_in_view(pool: web::Data<DbPool>) -> HttpResponse {
    // Récupération des données des événements
    let all_events = event_repository::paginate_events(pool, None, None);

    // Convertir les données des événements en une table
    let event_table = Table::from("event_table", all_events.clone());
    let event_list    = List::from("event_list", all_events.clone());

    println!("EVENT TABLE {:?}", event_table);

    let page_builder = PageBuilder::base_model(
        // NAVBAR
        "App title / logo",
        "Page title",
        Some(vec![
            ("Homepage".to_string(), "/".to_string()),
            ("Utilisateurs".to_string(), "/users".to_string()),
            ("Déconnexion".to_string(), "/users/logout".to_string()),
        ]),
        Some(vec![
            ("Utilisateurs".to_string(), "/users".to_string()),
            ("Déconnexion".to_string(), "/users/logout".to_string()),
        ]),
        // SECTION
        "Welcome Section",
        vec![
                DataType::Table(event_table),
                DataType::List(event_list)
            ], // Injecte le tableau dans la section
    );

    // Génération de l'html avec injection des données
    let html_output = generate_html::<_, EventItem>(
        "tera",
        all_events,
        page_builder,
    );

    // Retourner le HTML généré dans la réponse HTTP
    HttpResponse::Ok().content_type("text/html").body(html_output)
}


