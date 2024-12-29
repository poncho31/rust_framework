use std::process::exit;
use actix_files as fs;
use actix_web::{web};
use log::{info, warn};
use tera::Tera;
use crate::controllers::event_controller::{list_events, add_event};
use crate::controllers::user_controller::{list_users, add_user};
use crate::controllers::test_controller::{test_inject_object_in_view};
use crate::database;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        // EVENTS
        .service(web::resource("/")          .route(web::get() .to( list_events )))
        .service(web::resource("/add_event").route(web::post() .to( add_event   )))

        // USERS
        .service(web::resource("/users")   .route(web::get()  .to( list_users )))
        .service(web::resource("/add_user").route(web::post() .to( add_user   )))

        // TEST
        .service(web::resource("/test").route(web::get() .to( test_inject_object_in_view   )))


    ;
}

pub fn resources(cfg: &mut web::ServiceConfig) {
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

pub fn template_config(cfg: &mut web::ServiceConfig) {
    let engine = template_engine("tera"); // Par exemple, configurez Tera ici.
    configure_app(cfg, engine);
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
                exit(1);
            }
        }
    } else {
        warn!("Aucun moteur de template trouvé pour: {}", name);
        exit(1);
    }
}

