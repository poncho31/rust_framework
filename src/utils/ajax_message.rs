use actix_web::{HttpResponse, web};
use actix_web::web::Form;
use tera::Tera;
use crate::models::_models::NewEventData;

pub fn add_event_message(event_data : Form<NewEventData>, tmpl : web::Data<Tera>) -> HttpResponse{
    // Contexte pour un seul événement
    let mut context = tera::Context::new();
    context.insert("title", &event_data.title);
    context.insert("date", &event_data.date);
    context.insert("description", &event_data.description.as_deref().unwrap_or("Aucune description"));

    // Rendre la macro event_item pour cet événement
    let html_data = tmpl.render("html_module/shared/event_item_module_ajax.html", &context).expect("Erreur lors du rendu du template");

    HttpResponse::Ok().json(serde_json::json!({
                "status"        : "success",
                "message"       : "Événement ajouté avec succès.",
                "html_response" : html_data,
                "data"          : event_data
            }))
}