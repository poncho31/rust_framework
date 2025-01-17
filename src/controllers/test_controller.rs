use actix_web::{HttpResponse, web};
use crate::config::route_config::get_web_routes;
use crate::database::DbPool;
use crate::repository::event_repository;
use crate::utils::builder::page_builder::list::List;
use crate::utils::builder::page_builder::page_builder::PageBuilder;
use crate::utils::builder::page_builder::section::DataType;
use crate::utils::builder::page_builder::table::Table;
use crate::utils::template_engine::template::{generate_html};

pub async fn test_inject_object_in_view(pool: web::Data<DbPool>) -> HttpResponse {
    // Récupération des données des événements
    let all_events = event_repository::paginate_events(pool, None, Some(10));
    let table_event= Table::create(all_events.clone());
    let list_event   = List::create( all_events.clone());


    // Construction de l'objet PageBuilder
    let page_builder = PageBuilder::base_model(
        // NAVBAR
        "Rust framework",
        "Page title",
        Some(get_web_routes(Some("get"))),
        Some(get_web_routes(Some("get"))),
        // SECTION
        "Welcome Section",
        vec![
            DataType::Table(table_event),
            DataType::List(list_event)
            ], // Injecte le tableau dans la section
        3
    );

    // Génération de l'html avec injection des données
    let html_output = generate_html("tera", page_builder);

    // Retourner le HTML généré dans la réponse HTTP
    HttpResponse::Ok().content_type("text/html").body(html_output)
}


