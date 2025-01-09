use serde::Serialize;
use chrono::NaiveDate;
use tera::{Context, Tera};

/// Représente une liste structurée
#[derive(Serialize, Clone, Debug)]
pub struct List {
    pub id: String,
    pub items: Vec<ListItem>,
    pub template_file_path: String,
    pub css_file_path: Option<String>,
}

/// Représente un élément de liste avec des paires clé-valeur
#[derive(Serialize, Clone, Debug)]
pub struct ListItem {
    pub data: Vec<(String, String)>, // Paires (nom du champ, valeur)
}

impl List {
    pub fn new(id: &str, items: Vec<ListItem>) -> Self {
        Self {
            id: id.to_string(),
            items,
            template_file_path: "template/tera/list_tera.html".to_string(),
            css_file_path: Some("static/css/list.css".to_string()),
        }
    }

    /// Crée une liste de manière générique
    pub fn from<T: IntoList>(id: &str, data: Vec<T>) -> Self {
        let items: Vec<ListItem> = data.into_iter().map(|item| item.to_list_item()).collect();
        Self::new(id, items)
    }

    /// Rendu de la liste avec Tera
    pub fn render(&self, tera: &Tera) -> Result<String, tera::Error> {
        let mut context = Context::new();
        context.insert("content", &self);
        tera.render(&self.template_file_path, &context)
    }
}

/// Trait pour convertir un type en `ListItem`
pub trait IntoList {
    fn to_list_item(&self) -> ListItem;
}