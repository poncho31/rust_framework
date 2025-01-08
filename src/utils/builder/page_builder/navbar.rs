use serde_derive::Serialize;

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
    pub drop_down_menu : Option<Vec<(String, String)>>,
    pub shortcut_menu  : Option<Vec<(String, String)>>,
}
