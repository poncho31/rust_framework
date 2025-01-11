use serde_derive::Serialize;
use crate::config::route_config::RouteInfoDisplay;

#[derive(Serialize)]
pub struct NavBar {
    pub meta_data : NavBarMetadata,
    pub data      : NavBarData,
}
#[derive(Serialize)]
pub struct NavBarMetadata {
    pub file_name : String,
    pub raw_data  : NavBarData,
}

#[derive(Clone, Serialize)]
pub struct NavBarData {
    pub nav_title      : String,
    pub page_title     : String,
    pub drop_down_menu : Option<Vec<RouteInfoDisplay>>,
    pub shortcut_menu  : Option<Vec<RouteInfoDisplay>>,
}
