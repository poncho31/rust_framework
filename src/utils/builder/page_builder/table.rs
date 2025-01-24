use serde::{Deserialize, Serialize};
use crate::utils::common::generate_random_string;

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct Table {
    pub id                  : String,
    pub title               : String,
    pub headers             : Vec<String>,
    pub rows                : Vec<Vec<String>>,
    pub template_file_path  : String,
    pub css_file_path       : Option<String>,
}

impl Table {
    pub fn new(title: String, headers: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        Self {
            id: format!("id_table_{}",  generate_random_string(10)).parse().unwrap(),
            title,
            headers,
            rows,
            template_file_path : "template/tera/table_tera.html".to_string(),
            css_file_path      : Some("template".to_string()),
        }
    }

    // Fonction générique pour construire une table à partir d'une liste de données
    pub fn create<T: IntoHtmlTable>(title: &str,data: Vec<T>) -> Self {

        let headers : Vec<String>      = T::headers();
        let rows    : Vec<Vec<String>> = data.into_iter().map(|item| item.to_row()).collect();

        Self::new(title.to_string(),headers, rows)
    }
}

pub trait IntoHtmlTable {
    fn headers() -> Vec<String>;
    fn to_row(&self) -> Vec<String>;
}
