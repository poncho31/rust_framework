use crate::utils::builder::page::module::button::{Button};


// NAVBAR
pub struct NavBar {
    id: String,
    title: String,
    page_title: Option<String>,
    buttons: Option<Vec<Button>>,
    navbar_type:NavBarType
}

#[derive(Default)]
enum NavBarType{
    #[default]
    Default
}

impl NavBar {
    pub fn new(title: String,page_title: Option<String>, url: Option<String>, buttons : Option<Vec<Button>>) -> Self {
        NavBar {
            id: format!("navbar_{}", &title),
            title,
            page_title,
            buttons,
            navbar_type:NavBarType::default(),

        }
    }
}