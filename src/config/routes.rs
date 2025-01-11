use actix_web::{web};
use crate::config::config::RouteInfo;
use crate::controllers::event_controller::{add_event, list_events};
use crate::controllers::page_builder_controller::page_builder_view;
use crate::controllers::test_controller::test_inject_object_in_view;
use crate::controllers::user_controller::{add_user, list_users};


// Liste de routes centralisée
pub fn get_routes() -> Vec<RouteInfo> {
    vec![
        RouteInfo { name: "list_events", uri: "/",              method: web::get,  handler: Box::new(|| web::get().to(list_events)), },
        RouteInfo { name: "add_event",   uri: "/add_event",     method: web::post, handler: Box::new(|| web::post().to(add_event)), },
        RouteInfo { name: "list_users",  uri: "/users",         method: web::get,  handler: Box::new(|| web::get().to(list_users)), },
        RouteInfo { name: "add_user",    uri: "/add_user",      method: web::post, handler: Box::new(|| web::post().to(add_user)), },
        RouteInfo { name: "page_builder",uri: "/page/builder",  method: web::get,  handler: Box::new(|| web::get().to(page_builder_view)), },
        RouteInfo { name: "test",        uri: "/test",          method: web::get,  handler: Box::new(|| web::get().to(test_inject_object_in_view)), },
    ]
}