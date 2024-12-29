use actix_web::{HttpResponse, web};
use crate::database::DbPool;
use crate::repository::event_repository;
use crate::utils::builder::page::module::nav_bar::NavBar;
use crate::view::event_manager_view::EventManagerView;
use crate::view::event_table::{EventItem, EventTable};

pub async fn test_inject_object_in_view(pool: web::Data<DbPool>) -> HttpResponse {
    // Récupération des événements depuis la base de données
    let db_events = event_repository::paginate_events(pool, None, None);

    // Transformation des données de la base en `EventItem` pour EventTable
    let event_items: Vec<EventItem> = db_events.iter().map(|event| {
        EventItem {
            id: event.id.expect("ID manquant"),
            title: event.title.clone(),
            description: event.description.clone(),
            date: event.date.to_string(),
        }
    }).collect();

    // Création de la barre de navigation et de la table des événements
    let navbar = NavBar::new("MainNav".to_string(), Some("Événements".to_string()), None);
    let event_table = EventTable::new(event_items);

    // Utilisation d'EventManager pour générer le HTML
    let event_manager = EventManagerView::new(navbar, event_table);
    let html_output = event_manager.render_page();

    HttpResponse::Ok().content_type("text/html").body(html_output)
}