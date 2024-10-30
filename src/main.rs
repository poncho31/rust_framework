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
// Imports des modules internes (ton propre code)
use controllers::_event_controller::{list_events, add_event};
use controllers::_user_controller::{list_users, add_user};

use utils::{ env, server::web_server, server::web_view, server::server};

// Imports externes

use dotenv::dotenv;
use env_logger::Builder;

use std::io::Write;
use tao::platform::windows::IconExtWindows;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialiser l'environnement et le logger
    dotenv().ok();
    Builder::new()
        .filter(None, log::LevelFilter::Debug)
        .format(|buf, record| writeln!(buf, "[{}] - {}", record.level(), record.args()))
        .init();

    // Run server type from .env
    server::run().await
}


