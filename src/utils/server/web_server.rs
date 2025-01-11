use std::io::Result;
use actix_web::{web, App, HttpServer, middleware};

use log::{info};
use crate::{utils};
use crate::utils::command::execute;


pub async fn run(
        route_config    : fn(&mut web::ServiceConfig),
        resource_config : fn(&mut web::ServiceConfig),
        template_config : fn(&mut web::ServiceConfig)
    ) -> Result<()> {
    info!("Lancement en mode Serveur Web");

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())

            // CONFIGURATION
            .configure(move |cfg| template_config(cfg))

            // RESOURCES
            .configure(resource_config)

            // ROUTES
            .configure(route_config)
    })
        .workers(1)
        .bind(utils::env::get("APP_WEB_SERVER_URL"))?
        .run();

    let address = utils::env::get("PROXY_WEB_SERVER_URL");
    info!("Serveur Proxy Nginx en cours de démarrage à l'adresse : {}", address);
    start_proxy_server();

    if let Err(e) = server.await {
        eprintln!("Erreur lors de l'exécution du serveur Actix-Web: {}", e);
    }

    Ok(())
}



fn start_proxy_server() {
    let stop_service  = utils::env::get("PROXY_WEB_SERVER_STOP");
    let start_service = utils::env::get("PROXY_WEB_SERVER_START");

    execute::command("Stopping Nginx service", &stop_service);
    execute::command("Starting Nginx service", &start_service);
}


