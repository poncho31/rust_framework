// CMD :

//      Install :
//                rust               : curl https://sh.rustup.rs -sSf | sh
//                cargo watch        : cargo install cargo-watch
//                npm                : npm i | npm run build
//                diesel ORM install : cargo install diesel_cli --no-default-features --features sqlite-bundled
//                diesel config      : diesel setup

//      Command : cargo update
//                cargo build
//                cargo watch -x run
//                cargo watch -x "run" --poll (pour vérifier les fichiers à intervalles réguliers au lieu de se baser sur les événements du système de fichiers)
//                cargo watch -x "run" --why --ignore "db.sqlite-journal" (sinon le serveur redémarre dès qu'il y a un insert en DB)
//                diesel migration run
//                diesel print-schema

//      Documentation :
//                cargo doc --open
//                cargo doc --no-deps
//      Diagrams :
//                cargo modules structure --all-features                     (Affiche la structure hiérarchique)
//                cargo modules dependencies > storage/temp/dependencies.dot (Affiche les dépendances d'un module)
//                cargo modules orphans                                      (liste les modules orphelins)
//                cargo modules dependencies > storage/temp/dependencies.dot && dot -Tpng -Gdpi=300 storage/temp/dependencies.dot -o storage/temp/dependencies.png

//MODULES
    mod controllers;
    mod schema;
    mod models;
    mod repository;
    mod database;

    mod web_config;
    mod utils;
    mod view;

// Imports externes
    use dotenv::dotenv;
    use env_logger::Builder;
    use std::io::Write;

// Crates
    use crate::utils::{ server::server};
    use crate::web_config::{routes, resources, template_config};

// Lancement du serveur web
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialiser l'environnement et le logger
    dotenv().ok();
    Builder::new()
        .filter(None, log::LevelFilter::Debug)
        .format(|buf, record| writeln!(buf, "[{}] - {}", record.level(), record.args()))
        .init();

    // RUN WEB SERVER => injections des routes, resources et template depuis web_config.rs
    server::run(routes, resources, template_config).await
}