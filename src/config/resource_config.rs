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
        // STATIC FILE
        ResourceInfo { uri: "/resources/js",            local_path: "./resources/js",                             is_static_service: true, },
        ResourceInfo { uri: "/resources/css",           local_path: "./resources/css",                            is_static_service: true, },
        
        // NOT STATIC FILE      
        ResourceInfo { uri: "/favicon.ico",             local_path: "./resources/images/icons/favicon.ico",            is_static_service: false, },
        ResourceInfo { uri: "/resize_icon.png",         local_path: "./resources/images/icons/resize_icon.png",        is_static_service: false, },
        ResourceInfo { uri: "/file_icon.png",           local_path: "./resources/images/icons/file_icon.png",          is_static_service: false, },
        ResourceInfo { uri: "/folder_icon.png",         local_path: "./resources/images/icons/folder_icon.png",        is_static_service: false, },
        ResourceInfo { uri: "/default_icon.png",        local_path: "./resources/images/icons/default_icon.png",       is_static_service: false, },
        ResourceInfo { uri: "/shortcut_menu_icon.png",  local_path: "./resources/images/icons/shortcut_menu_icon.png", is_static_service: false, },

    ]
}