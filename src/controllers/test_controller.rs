use actix_web::{HttpResponse, web};
use crate::database::DbPool;
use crate::repository::event_repository;
use crate::utils::builder::page_builder::page_builder;
use crate::utils::builder::page_builder::page_builder::PageBuilder;
use crate::utils::template_engine::template::{generate_html};
use crate::view::event_table::{EventItem};

pub async fn test_inject_object_in_view(pool: web::Data<DbPool>) -> HttpResponse {
    // Récupération des données
    let all_events = event_repository::paginate_events(pool, None, None);

    let page_builder = PageBuilder::base_model(
        "Custom Event Manager",
        true,
        Some(vec![
            ("Utilisateurs".to_string(), "/users".to_string()),
            ("Déconnexion".to_string(), "/users/logout".to_string()),
        ]),
        "Welcome Section",
        "This is the main content of the page.",
    );

    // Génération de l'html avec injection des données
    let html_output = generate_html::<_, EventItem>(
        "tera",
        all_events,
        page_builder
    );

    // Retourner le HTML généré dans la réponse HTTP
    HttpResponse::Ok().content_type("text/html").body(html_output)
}

