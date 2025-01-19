use crate::models::user_model::NewUserData;
use crate::database::get_connection;
use crate::repository::user_repository;
use crate::utils::ajax_message::add_user_message;

use actix_web::{web, HttpResponse};
use log::{debug, info, warn};
use tera::Tera;
use crate::config::route_config::get_web_routes;
use crate::utils::builder::page_builder::page_builder::PageBuilder;
use crate::utils::builder::page_builder::section::DataType;
use crate::utils::builder::page_builder::table::Table;
use crate::utils::template_engine::template::generate_html;

pub async fn list_users(pool: web::Data<crate::database::DbPool>) -> HttpResponse {

    // Récupération des données des événements
    let all_users = user_repository::paginate_users(pool, None, Some(100));

    let section_content = vec![
        DataType::Table(Table::create("Table",all_users.clone())),
    ];

    // Construction de l'objet PageBuilder
    let page_builder = PageBuilder::base_model(
        // NAVBAR
        "Rust framework",
        "Utilisateurs",
        Some(get_web_routes(Some("get"))),
        Some(get_web_routes(Some("get"))),
        // SECTION
        "Utilisateurs du portail",
        section_content.clone()
    );

    // Génération de l'html avec injection des données
    let html_output = generate_html("tera", page_builder);

    // Retourner le HTML généré dans la réponse HTTP
    HttpResponse::Ok().content_type("text/html").body(html_output)
}

pub async fn add_user(user_data: web::Form<NewUserData>, pool: web::Data<crate::database::DbPool>, tmpl: web::Data<Tera>) -> HttpResponse {
    debug!("Début de la fonction add_user...");

    let mut conn = match get_connection(pool) {
        Ok(conn) => conn,
        Err(err_response) => return err_response,  // En cas d'échec, retourner l'erreur HTTP
    };

    let new_user = &user_data.to_new_user();

    match user_repository::insert_user(&new_user, &mut conn) {
        Ok(_) => {
            info!("Utilisateur ajouté avec succès.");
            add_user_message(user_data, tmpl)
        },
        Err(e) => {
            warn!("Erreur lors de l'ajout de l'utilisateur : {:?}", e);
            HttpResponse::InternalServerError().json("Failed to add user")
        }
    }
}