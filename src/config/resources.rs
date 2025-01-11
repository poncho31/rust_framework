use crate::config::config::ResourceInfo;

pub fn get_resources() -> Vec<ResourceInfo> {
    vec![
        ResourceInfo { uri: "/resources/js",  local_path: "./resources/js",                      is_service: true, },
        ResourceInfo { uri: "/resources/css", local_path: "./resources/css",                      is_service: true, },
        ResourceInfo { uri: "/favicon.ico",   local_path: "./resources/images/icons/favicon.ico", is_service: false, },
    ]
}