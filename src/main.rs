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
//                cargo watch -x "run" --why --ignore "db.sqlite-journal" --ignore "resources/**"
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

// MODULES
mod controllers;
mod schema;
mod models;
mod repository;
mod database;
mod utils;
mod config;

// Imports externes
use dotenv::dotenv;
use env_logger::Builder;
use std::io::Write;
use std::process::Command;

// Crates et imports de modules locaux
use utils::server::browser;
use crate::utils::server::server;
use crate::config::{config::route_config, config::template_config, config::resource_config};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Vérifie si l'argument "--browser" est présent
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "--browser" {
        // Exécute uniquement le navigateur TUI
        browser::browser_run();
        return Ok(());
    }

    // Sinon, dans le processus principal, on lance le navigateur dans un nouveau terminal.
    let exe = std::env::current_exe().expect("Impossible d'obtenir le chemin de l'exécutable");
    #[cfg(target_os = "windows")]
    {
        // Sur Windows, on utilise "cmd" et "start" pour ouvrir une nouvelle fenêtre.
        Command::new("cmd")
            .args(&["/C", "start", "", exe.to_str().unwrap(), "--browser"])
            .spawn()
            .expect("Échec du lancement du navigateur dans un nouveau terminal");
    }
    #[cfg(not(target_os = "windows"))]
    {
        // Sur Unix, on utilise "xterm". Vous pouvez adapter (ex: gnome-terminal, konsole, etc.)
        Command::new("xterm")
            .arg("-e")
            .arg(format!("{} --browser", exe.to_str().unwrap()))
            .spawn()
            .expect("Échec du lancement du navigateur dans un nouveau terminal");
    }

    // Initialisation de l'environnement
    dotenv().ok();
    Builder::new()
        .filter(None, log::LevelFilter::Debug)
        .format(|buf, record| writeln!(buf, "[{}] - {}", record.level(), record.args()))
        .init();

    // Lancement du serveur web avec injection des configurations depuis config.rs
    server::run(route_config, resource_config, template_config).await
}