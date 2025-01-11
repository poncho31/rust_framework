use actix_web::{Route, web};
use serde_derive::Serialize;
use crate::config::update::routes::web_routes;
use crate::controllers::event_controller::{add_event, list_events};
use crate::controllers::page_builder_controller::page_builder_view;
use crate::controllers::test_controller::test_inject_object_in_view;
use crate::controllers::user_controller::{add_user, list_users};

// ROUTES APP
pub fn get_routes()->Vec<RouteInfo>{
    let web_routes = if web_routes().is_empty() { web_routes_default() } else { web_routes() };
    web_routes
}


pub struct RouteInfo {
    pub name   : &'static str,
    pub uri    : &'static str,
    pub method : &'static str,
    pub handler: Box<dyn Fn() -> Route + Send + Sync>,
}

impl std::fmt::Debug for RouteInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RouteInfo")
            .field("name", &self.name)
            .field("uri", &self.uri)
            .field("method", &self.method)
            .finish() // Ne pas inclure `handler`
    }
}

pub fn web_routes_default() -> Vec<RouteInfo> {
    vec![
        RouteInfo { name: "Liste événement",   uri: "/",              method: "get", handler: Box::new(|| web::get().to(list_events)),                },
        RouteInfo { name: "add_event",         uri: "/add_event",     method: "post",handler: Box::new(|| web::post().to(add_event)),                 },
        RouteInfo { name: "users",             uri: "/users",         method: "get", handler: Box::new(|| web::get().to(list_users)),                  },
        RouteInfo { name: "add_user",          uri: "/add_user",      method: "post",handler: Box::new(|| web::post().to(add_user)),                  },
        RouteInfo { name: "Page builder",      uri: "/page/builder",  method: "get", handler: Box::new(|| web::get().to(page_builder_view)),          },
        RouteInfo { name: "Test",              uri: "/test",          method: "get", handler: Box::new(|| web::get().to(test_inject_object_in_view)), },
    ]
}

// ROUTES VUE
#[derive(Clone, Serialize)]
pub struct RouteInfoDisplay {
    pub name   : String,
    pub uri    : String,
    pub method : String,
}

pub fn get_web_routes(filter_method: Option<&str>) -> Vec<RouteInfoDisplay> {
    get_routes()
        .into_iter()
        .filter(|route| {
            if let Some(method) = filter_method {
                route.method == method // Filtre sur le type de méthode
            } else {
                true // Aucun filtre si `filter_method` est `None`
            }
        })
        .map(|route| RouteInfoDisplay {
            name   : route.name.to_string(),
            uri    : route.uri.to_string(),
            method : route.method.to_string(),
        })
        .collect()
}

