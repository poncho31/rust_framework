// CMD :
//      diesel migration run
//      cargo watch -x run
//      npm i | npm run build


mod controllers; // Import des contrôleurs
mod schema;      // Import du schéma généré par Diesel
mod models;

use actix_files as fs;
use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use tera::Tera;

use crate::controllers::_event_controller::{list_events, add_event, show_add_event_form};

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialisation du moteur de templates Tera
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/views/**/*")).unwrap();

    // Démarrage du serveur HTTP
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(establish_connection_pool())) // Pool de connexions
            .app_data(web::Data::new(tera.clone())) // Moteur de templates
            .service(fs::Files::new("/resources/js", "./resources/js").show_files_listing())
            .service(fs::Files::new("/resources/css", "./resources/css").show_files_listing())
            .service(add_event) // Route pour ajouter un événement
            .service(list_events) // Route pour lister les événements
            .service(show_add_event_form) // Route pour afficher le formulaire d'ajout d'événement
    })
    .bind("127.0.0.1:8082")? // Serveur lié à l'adresse et au port
    .run()
    .await
}

fn establish_connection_pool() -> DbPool {
    let manager = ConnectionManager::<SqliteConnection>::new("db.sqlite");
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.") // Gestion d'erreur si la pool échoue
}
