use actix_web::{HttpResponse};
use crate::utils::builder::page_builder::page_builder::{page_builder_exemple};
use crate::utils::template_engine::template::generate_html;

// pool: web::Data<DbPool>
pub async fn page_builder_view() -> HttpResponse {

    // Génération de l'html avec injection des données
    let html_output = generate_html("tera", page_builder_exemple());

    // Retourner le HTML généré dans la réponse HTTP
    HttpResponse::Ok().content_type("text/html").body(html_output)
}