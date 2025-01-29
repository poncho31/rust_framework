use std::process::exit;
use actix_files as fs;
use actix_web::web;
use actix_web::http::Method;
use log::{info, warn};
use tera::Tera;
use crate::config::resource_config::get_resources;
use crate::config::route_config::get_routes;
use crate::database;
use crate::utils::env::get;

// ROUTES
pub fn route_config(cfg: &mut web::ServiceConfig) {
    for route in get_routes() {
        let method = match route.method.to_uppercase().as_str() {
            "GET"    => Method::GET,
            "POST"   => Method::POST,
            "PUT"    => Method::PUT,
            "DELETE" => Method::DELETE,
            _ => continue,
        };

        let uri = if method == Method::GET {
            route.uri.to_string()
        } else {
            format!("/{}/{}", route.method.to_lowercase(), route.uri.trim_start_matches('/'))
        };

        cfg.service(web::resource(uri).route((route.handler)().method(method)));
    }
}


// RESOURCES
pub fn resource_config(cfg: &mut web::ServiceConfig) {
    for resource in get_resources() {
        if resource.is_static_service {
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

// TEMPLATE
pub fn template_config(cfg: &mut web::ServiceConfig) {
    configure_app(cfg, template_engine("tera"))
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

