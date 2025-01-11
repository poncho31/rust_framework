use std::process::exit;
use actix_files as fs;
use actix_web::{Route, web};
use log::{info, warn};
use tera::Tera;
use crate::config::resources::get_resources;
use crate::config::routes::get_routes;
use crate::controllers::event_controller::{list_events, add_event};
use crate::controllers::user_controller::{list_users, add_user};
use crate::controllers::test_controller::{test_inject_object_in_view};
use crate::controllers::page_builder_controller::{page_builder_view};
use crate::database;
use crate::utils::env::get;

/// ROUTES
pub struct RouteInfo {
    pub name   : &'static str,
    pub uri    : &'static str,
    pub method : fn() -> Route,
    pub handler: Box<dyn Fn() -> Route + Send + Sync>,
}
pub fn routes(cfg: &mut web::ServiceConfig) {
    for route in get_routes() {
        cfg.service(
            web::resource(route.uri)
                .route((route.handler)()) // Utilisation de la route encapsulée
                .name(route.name),
        );
    }
}

/// RESOURCES
pub struct ResourceInfo {
    pub uri: &'static str,
    pub local_path: &'static str,
    pub is_service: bool, // Détermine si c'est un service statique ou une ressource dynamique
}
pub fn resources(cfg: &mut web::ServiceConfig) {
    for resource in get_resources() {
        if resource.is_service {
            // Gestion des fichiers statiques via `fs::Files`
            cfg.service(
                fs::Files::new(resource.uri, resource.local_path).show_files_listing(),
            );
        } else {
            // Gestion des ressources dynamiques (images, fichiers, etc.)
            let local_path = resource.local_path.to_string();
            cfg.route(
                resource.uri,
                web::get().to(move |req: actix_web::HttpRequest| {
                    let local_path = local_path.clone();
                    async move {
                        fs::NamedFile::open_async(local_path)
                            .await
                            .unwrap()
                            .into_response(&req)
                    }
                }),
            );
        }
    }
}



pub fn template_config(cfg: &mut web::ServiceConfig) {
    let engine = template_engine("tera");
    configure_app(cfg, engine);
}

fn template_engine(name: &str) -> Tera {
    if name == "tera" {
        // Récupérer le chemin depuis le fichier .env
        let views_path = get("RESOURCES_VIEWS_PATH");

        match Tera::new(&format!("{}/**/*", views_path)) {
            Ok(t) => {
                info!("Moteur template Tera initialisé avec succès.");
                t.clone() // Retourner l'objet `Tera` en cas de succès
            }
            Err(e) => {
                warn!("Erreur lors de l'initialisation du Moteur template Tera : {:?}", e);
                exit(1);
            }
        }
    } else {
        warn!("Aucun moteur de template trouvé pour: {}", name);
        exit(1);
    }
}


fn configure_app(cfg: &mut web::ServiceConfig, tera: Tera) {
    cfg
        // Pool de connexions
        .app_data(web::Data::new(database::establish_connection_pool()))

        // Moteur de templates
        .app_data(web::Data::new(tera))
    ;
}

