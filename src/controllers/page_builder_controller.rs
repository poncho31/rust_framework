use actix_web::{HttpResponse, web};
use crate::config::route_config::get_web_routes;
use crate::database::DbPool;
use crate::repository::event_repository;
use crate::utils::builder::page_builder::list::List;
use crate::utils::builder::page_builder::page_builder::PageBuilder;
use crate::utils::builder::page_builder::section::DataType;
use crate::utils::builder::page_builder::table::Table;
use crate::utils::template_engine::template::generate_html;


pub async fn page_builder_view(pool: web::Data<DbPool>) -> HttpResponse {
    let all_events = event_repository::paginate_events(pool, None, Some(100));

    /// Construction de l'objet PageBuilder
    let page_builder = PageBuilder::base_model(
        /// NAVBAR
        "Rust framework",
        "Page builder",
        Some(get_web_routes(Some("get"))),
        Some(get_web_routes(Some("get"))),
        /// SECTION
        "",
        vec![
            DataType::Table(Table::from("event_table", all_events.clone())),
            DataType::List(List::from("event_list", all_events.clone()))
        ], // Injecte le tableau dans la section
    );

    /// Génération de l'html avec injection des données
    let html_output = generate_html("tera", page_builder);

    // Retourner le HTML généré dans la réponse HTTP
    HttpResponse::Ok().content_type("text/html").body(html_output)
}