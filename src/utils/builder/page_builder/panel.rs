use serde::{Deserialize, Serialize};
use crate::utils::common::generate_random_string;

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct Panel {
    pub id                  : String,
    pub title               : String,
    pub menu                : Vec<String>,
    pub template_file_path  : String,
    pub css_file_path       : Option<String>,
}

impl Panel {
    pub fn new(title: String, menu: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        Self {
            id: format!("id_Panel_{}",  generate_random_string(10)).parse().unwrap(),
            title,
            menu,
            rows,
            template_file_path : "template/tera/panel_tera.html".to_string(),
            css_file_path      : Some("template".to_string()),
        }
    }

    // Fonction générique pour construire une Panel à partir d'une liste de données
    pub fn create<T: IntoHtmlPanel>(title: &str,data: Vec<T>) -> Self {

        let menu : Vec<String>      = T::menu();
        let rows    : Vec<Vec<String>> = data.into_iter().map(|item| item.to_row()).collect();

        Self::new(title.to_string(),menu, rows)
    }
}

pub trait IntoHtmlPanel {
    fn menu() -> Vec<String>;
    fn to_row(&self) -> Vec<String>;
}
