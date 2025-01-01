use std::collections::HashMap;
use actix_web::{HttpResponse, web};
use tera::{Context, Tera};
use crate::database::DbPool;
use crate::repository::event_repository;
use crate::utils::builder::page::module::nav_bar::NavBar;
use crate::view::event_table::{EventItem};
use crate::utils::transform::db_transform::{get_collection_data};

pub async fn test_inject_object_in_view(pool: web::Data<DbPool>) -> HttpResponse {
    // Récupération des événements depuis la base de données
    let all_events = event_repository::paginate_events(pool, None, None);
    println!("{:#?}", "ALL TEST");
    println!("{:#?}", all_events);

    let data: Vec<EventItem> = get_collection_data(&all_events);
    println!("{:#?}", "TEST ITEM");
    println!("{:#?}", data);

    // TODO : continuer la modification ici
    let mut html_map: HashMap<&str, String> = HashMap::new();
    html_map.insert("html_navbar", html_navbar());
    html_map.insert("html_section", html_section());
    html_map.insert("html_footer", html_footer());

    let html_output = template_tera(html_map, "template/base_tera_template.html");

    // Retourner le HTML généré dans la réponse HTTP
    HttpResponse::Ok().content_type("text/html").body(html_output)
}

use serde_json::to_string_pretty;




pub fn html_navbar()->String{
    let navbar = NavBar::new("MainNav".to_string(), Some("Événements".to_string()), None);
    navbar.to_html()
}

pub fn html_section()->String{
    "<div>SECTION</div>".to_string()
}

pub fn html_footer()->String{

    "<div>FOOTER</div>".to_string()
}

pub fn html_error()->String{
    "<div>ERROR</div>".to_string()
}

pub fn template_tera(html: HashMap<&str, String>, template_html_path: &str) -> String {
    let mut context = Context::new();

    // Html sections
    for (key, value) in &html {
        context.insert(*key, value);
    }

    // Html debug
    context.insert("debug_tera_context", &tera_debug_context(&context));

    // Initialiser Tera
    let tera = Tera::new("resources/views/**/*").unwrap_or_else(|e| {
        println!("Erreur lors du chargement des templates : {:?}", e);
        std::process::exit(1);
    });

    // Rendu du template
    match tera.render(template_html_path, &context) {
        Ok(rendered_html) => {
            println!("HTML rendu :\n{}", rendered_html);
            rendered_html
        }
        Err(e) => {
            println!("Erreur lors du rendu du template : {:?}", e);
            html_error().to_string()
        }
    }
}
pub fn tera_debug_context(context: &Context) -> String {
    let json_value = context.clone().into_json();
    to_string_pretty(&json_value).unwrap_or_else(|_| "Erreur lors de la conversion du contexte".to_string())
}
