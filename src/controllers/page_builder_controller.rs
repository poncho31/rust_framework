use actix_web::{HttpResponse, web};
use crate::database::DbPool;
use crate::utils::builder::page_builder::page_builder::{page_builder_exemple};
use crate::utils::template_engine::template::generate_html;


pub async fn page_builder_view(pool: web::Data<DbPool>) -> HttpResponse {

    // Génération de l'html avec injection des données
    let html_output = generate_html("tera", page_builder_exemple(pool));

    // Retourner le HTML généré dans la réponse HTTP
    HttpResponse::Ok().content_type("text/html").body(html_output)
}

pub async fn add_page_builder(pool: web::Data<DbPool>) -> HttpResponse {
    HttpResponse::Ok().content_type("application/json").body("{'RESPONSE':'OOOKKKKK'}")
}
