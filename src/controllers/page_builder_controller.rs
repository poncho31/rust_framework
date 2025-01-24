use actix_web::{HttpResponse, web};
use crate::database::DbPool;
use crate::utils::builder::page_builder::form::Form;
use crate::utils::builder::page_builder::page_builder::{page_builder_exemple, PageBuilder};
use crate::utils::template_engine::template::generate_html;


pub async fn page_builder_view(pool: web::Data<DbPool>) -> HttpResponse {

    // Génération de l'html avec injection des données
    let html_output = generate_html("tera", page_builder_exemple(pool));

    // Retourner le HTML généré dans la réponse HTTP
    HttpResponse::Ok().content_type("text/html").body(html_output)
}

pub async fn add_page_builder(request: web::Json<PageBuilder>) -> HttpResponse {
    // Extraire l'objet PageBuilder de la requête
    let existing_page_builder = request.into_inner();

    print!("{:?}", existing_page_builder);

    // Créer un nouvel objet PageBuilder basé sur celui reçu
    let new_page_builder = PageBuilder::create_from_request(existing_page_builder);

    // Retourner l'objet JSON du nouveau PageBuilder
    HttpResponse::Ok().json(new_page_builder)
}
