use serde::Deserialize;
use serde_derive::Serialize;
use crate::config::route_config::RouteInfoDisplay;

#[derive(Serialize, Deserialize, Debug)]
pub struct NavBar {
    pub file_name : String,

    pub nav_title      : String,
    pub page_title     : String,
    pub drop_down_menu : Option<Vec<RouteInfoDisplay>>,
    pub shortcut_menu  : Option<Vec<RouteInfoDisplay>>,
}