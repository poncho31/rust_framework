use serde_derive::Serialize;

#[derive(Serialize)]
pub struct NavBar {
    pub debug_data   : NavBarDebug,
    pub template_data: NavBarData,
}
#[derive(Serialize)]
pub struct NavBarDebug {
    pub file_path : String,
    pub raw_data  : NavBarData,
}
#[derive(Clone, Serialize)]
pub struct NavBarData {
    pub title          : String,
    pub page_title     : String,
    pub drop_down_menu : Option<Vec<(String, String)>>,
}
