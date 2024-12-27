use actix_files as fs;
use actix_web::{web};
use crate::controllers::_event_controller::{list_events, add_event};
use crate::controllers::_user_controller::{list_users, add_user};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        // EVENTS
        .service(web::resource("/")          .route(web::get() .to( list_events )))
        .service(web::resource("/add_event").route(web::post() .to( add_event   )))

        // USERS
        .service(web::resource("/users")   .route(web::get()  .to( list_users )))
        .service(web::resource("/add_user").route(web::post() .to( add_user   )))
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
