use crate::config::update::resources::web_resources;

pub fn get_resources() ->Vec<ResourceInfo>{
    let web_resources = if web_resources().is_empty() { web_resources_default() } else { web_resources() };
    web_resources
}

pub struct ResourceInfo {
    pub uri: &'static str,
    pub local_path: &'static str,
    pub is_static_service: bool, // Détermine si c'est un service statique ou une ressource dynamique
}

pub fn web_resources_default() -> Vec<ResourceInfo> {
    vec![
        ResourceInfo { uri: "/resources/js",  local_path: "./resources/js",                       is_static_service: true, },
        ResourceInfo { uri: "/resources/css", local_path: "./resources/css",                      is_static_service: true, },
        ResourceInfo { uri: "/favicon.ico",   local_path: "./resources/images/icons/favicon.ico", is_static_service: false, },
    ]
}