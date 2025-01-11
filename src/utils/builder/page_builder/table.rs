use serde::Serialize;
use crate::utils::common::generate_random_string;

#[derive(Serialize, Clone, Debug)]
pub struct Table {
    pub id: String,
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub template_file_path: String,
    pub css_file_path: Option<String>,
}

impl Table {
    pub fn new(id: &str, headers: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        Self {
            id: format!("{}_{}", id.to_string(), generate_random_string(10)).parse().unwrap(),
            headers,
            rows,
            template_file_path: "template/tera/table_tera.html".to_string(),
            css_file_path: Some("template".to_string()),
        }
    }

    /// Fonction générique pour construire une table à partir d'une liste de données
    pub fn from<T: IntoHtmlTable>(id: &str, data: Vec<T>) -> Self {
        let headers = T::headers();
        let rows: Vec<Vec<String>> = data.into_iter().map(|item| item.to_row()).collect();
        Self::new(id, headers, rows)
    }
}

pub trait IntoHtmlTable {
    fn headers() -> Vec<String>;
    fn to_row(&self) -> Vec<String>;
}
