use serde::{Deserialize, Serialize};
use crate::utils::common::generate_random_string;

// Représente une liste structurée
#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct List {
    pub id: String,
    pub title: String,
    pub items: Vec<ListItem>,
    pub template_file_path: String,
    pub css_file_path: Option<String>,
}

// Représente un élément de liste avec des paires clé-valeur
#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct ListItem {
    pub data: Vec<(String, String)>, // Paires (nom du champ, valeur)
}

impl List {
    pub fn new(title: String,items: Vec<ListItem>) -> Self {
        Self {
            id: format!("id_list_{}", generate_random_string(10)).parse().unwrap(),
            title,
            items,
            template_file_path: "template/tera/list_tera.html".to_string(),
            css_file_path: Some("static/css/list.css".to_string()),
        }
    }

    // Crée une liste de manière générique
    pub fn create<T: IntoHtmlList>(title: &str,data: Vec<T>) -> Self {
        let items: Vec<ListItem> = data.into_iter().map(|item| item.to_list_item()).collect();

        Self::new(title.to_string(),items)
    }
}

// Trait pour convertir un type en `ListItem`
pub trait IntoHtmlList {
    fn to_list_item(&self) -> ListItem;
}