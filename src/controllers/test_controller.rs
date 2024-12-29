use actix_web::{HttpResponse, web};
use crate::database::DbPool;
use crate::repository::event_repository;
use crate::utils::builder::page::module::nav_bar::NavBar;
use crate::view::test_view::TestView;
use crate::view::event_table::{EventItem, EventTable};
use crate::utils::transform::db_transform::{FromDbRow, get_collection_data};

pub async fn test_inject_object_in_view(pool: web::Data<DbPool>) -> HttpResponse {
    // Récupération des événements depuis la base de données
    let all_events = event_repository::paginate_events(pool, None, None);
    println!("{:#?}", "ALL EVENTS");
    println!("{:#?}", all_events);

    let event_items: Vec<EventItem> = get_collection_data(&all_events);
    println!("{:#?}", "EVENT ITEM");
    println!("{:#?}", event_items);

    // Création de la barre de navigation et de la table des événements
    let navbar = NavBar::new("MainNav".to_string(), Some("Événements".to_string()), None);
    let event_table = EventTable::new(event_items);

    // Utilisation d'EventManager pour générer le HTML
    let event_manager = TestView::new(navbar, event_table);
    let html_output = event_manager.render_page();

    HttpResponse::Ok().content_type("text/html").body(html_output)
}