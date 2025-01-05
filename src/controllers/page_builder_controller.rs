use actix_web::{HttpResponse, web};
use crate::database::DbPool;


pub async fn page_builder_view(pool: web::Data<DbPool>) -> HttpResponse {
    // Génération de l'html avec injection des données
    let html_output = "";

    // Retourner le HTML généré dans la réponse HTTP
    HttpResponse::Ok().content_type("text/html").body(html_output)
}