// CMD :
//      install :
//                rust               : curl https://sh.rustup.rs -sSf | sh
//                cargo watch        : cargo install cargo-watch
//                npm                : npm i | npm run build
//                diesel ORM install : cargo install diesel_cli --no-default-features --features sqlite-bundled
//                diesel config      : diesel setup
//      diesel migration run
//      cargo update
//      cargo watch -x run
//      cargo watch -x "run" --poll (pour vérifier les fichiers à intervalles réguliers au lieu de se baser sur les événements du système de fichiers)
//      cargo watch -x "run" --why --ignore "db.sqlite-journal" (sinon le serveur redémarre dès qu'il y a un insert en DB)


//MODULES
mod controllers;
mod schema;
mod models;
mod repository;
mod database;

mod utils;

// CRATE CONTROLLERS
use crate::controllers::_event_controller::{list_events, add_event};
// use crate::controllers::_user_controller::{register, login};

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

    info!("Serveur en cours de démarrage...");


    // Démarrage du serveur HTTP
    info!("Initialisation dela configuration web, des routes et des fichiers statiques...");
    HttpServer::new(move || {
        App::new().wrap(middleware::Logger::default())

            // Configure app
            .configure(|cfg| configure_app(cfg, template_engine("tera")))

            // Ajouts des fichiers statiques : css, js, images, .ico
            .configure(resources)

            // ROUTES
            .configure(routes)
    })
    .workers(1)              // Par défaut, Actix crée autant de threads que le nombre de cœurs disponibles sur ton processeur. Si tu n'as pas explicitement défini le nombre de workers, chaque thread pourrait réinitialiser la configuration de l'application, y compris l'appel à establish_connection_pool()
    .bind("127.0.0.1:8082")? // Serveur lié à l'adresse et au port
    .run()
    .await
}

fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        // EVENTS
        .service(add_event)
        .service(list_events);
}

fn resources(cfg: &mut web::ServiceConfig){
    cfg
        // JS
        .service(fs::Files::new("/resources/js", "./resources/js").show_files_listing())

        // CSS
        .service(fs::Files::new("/resources/css", "./resources/css").show_files_listing())

        // RESOURCE IMAGE .ico
        .route("/favicon.ico", web::get().to(|| async {
            fs::NamedFile::open_async("./resources/images/icons/favicon.ico").await.unwrap()
        }));
}

fn configure_app(cfg: &mut web::ServiceConfig, tera: Tera) {
    cfg
        // Pool de connexions
        .app_data(web::Data::new(database::establish_connection_pool()))

        // Moteur de templates
        .app_data(web::Data::new(tera));
}

fn template_engine(name: &str) -> Tera {
    if name == "tera" {
        match Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/views/**/*")) {
            Ok(t) => {
                info!("Moteur template Tera initialisé avec succès.");
                t.clone()  // Retourner l'objet `Tera` en cas de succès
            }
            Err(e) => {
                warn!("Erreur lors de l'initialisation du Moteur template Tera : {:?}", e);
                std::process::exit(1);
            }
        }
    } else {
        warn!("Aucun moteur de template trouvé pour: {}", name);
        std::process::exit(1);
    }
}
