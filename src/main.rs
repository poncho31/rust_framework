// CMD :
//      install :
//                rust               : curl https://sh.rustup.rs -sSf | sh
//                cargo watch        : cargo install cargo-watch
//                npm                : npm i | npm run build
//                diesel ORM install : cargo install diesel_cli --no-default-features --features sqlite-bundled
//                diesel config      : diesel setup
//      diesel migration run
//      cargo watch -x run
//      cargo watch -x "run" --poll (pour vérifier les fichiers à intervalles réguliers au lieu de se baser sur les événements du système de fichiers)
//      cargo watch -x "run" --why --ignore "db.sqlite-journal" (sinon le serveur redémarre dès qu'il y a un insert en DB)

//MODULES
mod controllers; // Import des contrôleurs
mod schema;      // Import du schéma généré par Diesel
mod models;

// CRATE CONTROLLERS
use crate::controllers::_event_controller::{list_events, add_event, show_add_event_form};
use crate::controllers::_user_controller::{register, login};

// CRATE
use std::io::Write;
use actix_files as fs;
use actix_web::{web, App, HttpServer, middleware};
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use tera::Tera;
use log::{info, warn, debug}; // Import des macros de log
use env_logger::Builder;       // Utilisation explicite de Builder pour configurer les logs

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;


//  SERVEUR
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialisation du logger avec Builder pour forcer les logs à s'afficher
    Builder::new()
            .filter(None, log::LevelFilter::Debug) // Filtre pour afficher tous les logs au niveau Debug ou supérieur
            .format(|buf, record| writeln!(buf, "[{}] - {}", record.level(), record.args())) // Format des logs
            .init();

    // Log manuel pour tester l'initialisation des logs
    info!("Serveur en cours de démarrage...");

    // Initialisation du moteur de templates Tera
    let tera = match Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/views/**/*")) {
        Ok(t) => {
            info!("Moteur template Tera initialisé avec succès.");
            t
        },
        Err(e) => {
            warn!("Erreur lors de l'initialisation du Moteur template Tera : {:?}", e);
            std::process::exit(1);
        }
    };

    // Démarrage du serveur HTTP
    info!("Initialisation des routes et des fichiers statiques...");
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default()) // Middleware Logger
            .app_data(web::Data::new(establish_connection_pool())) // Pool de connexions
            .app_data(web::Data::new(tera.clone())) // Moteur de templates

            // Ajouts des fichiers statiques : css, js, images, .ico
            .service(fs::Files::new("/resources/js", "./resources/js").show_files_listing())
            .service(fs::Files::new("/resources/css", "./resources/css").show_files_listing())
            .route("/favicon.ico", web::get().to(|| async {
                        fs::NamedFile::open_async("./resources/images/icons/favicon.ico").await.unwrap()
                    }))

            // ROUTES
            .service(add_event)
            .service(list_events)
            .service(show_add_event_form)
    })
    .workers(1)              // Par défaut, Actix crée autant de threads que le nombre de cœurs disponibles sur ton processeur. Si tu n'as pas explicitement défini le nombre de workers, chaque thread pourrait réinitialiser la configuration de l'application, y compris l'appel à establish_connection_pool()
    .bind("127.0.0.1:8082")? // Serveur lié à l'adresse et au port
    .run()
    .await
}

fn establish_connection_pool() -> DbPool {
    let manager = ConnectionManager::<SqliteConnection>::new("db.sqlite");

    // Log manuel pour tester l'initialisation de la base de données
    info!("Initialisation du pool de connexions à la base de données...");

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.") // Gestion d'erreur si la pool échoue
}
