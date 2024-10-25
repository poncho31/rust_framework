use actix_web::{HttpResponse, web};
use actix_web::web::Form;
use tera::Tera;
use crate::models::_models::{NewEventData, NewUserData};

pub fn add_event_message(event_data : Form<NewEventData>, tmpl : web::Data<Tera>) -> HttpResponse{
    // Contexte pour un seul événement
    let mut context = tera::Context::new();
    context.insert("title",       &event_data.title);
    context.insert("date",        &event_data.date);
    context.insert("description", &event_data.description.as_deref().unwrap_or("Aucune description"));

    // Rendre la macro event_item pour cet événement
    let html_data = tmpl.render("event/event_item_module_ajax.html", &context).expect("Erreur lors du rendu du template");

    HttpResponse::Ok().json(serde_json::json!({
                "status"        : "success",
                "message"       : "Événement ajouté avec succès.",
                "html_response" : html_data,
                "data"          : event_data
            }))
}

pub fn add_user_message(user_data : Form<NewUserData>, tmpl : web::Data<Tera>) -> HttpResponse{
    // Contexte pour un seul événement
    let mut context = tera::Context::new();
    context.insert("username",   &user_data.username);
    // If created_at is an Option<String>
    context.insert(
        "created_at",
        &user_data
            .created_at
            .as_deref() // Get &str from Option<String>
            .unwrap_or("/"), // Use "/" if None
    );
    context.insert("email",      &user_data.email);

    // Rendre la macro event_item pour cet événement
    let html_data = tmpl.render("user/user_item_module_ajax.html", &context).expect("Erreur lors du rendu du template");

    HttpResponse::Ok().json(serde_json::json!({
                "status"        : "success",
                "message"       : "Événement ajouté avec succès.",
                "html_response" : html_data,
                "data"          : user_data
            }))
}