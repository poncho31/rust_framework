use std::collections::HashMap;
use serde::Serialize;
use crate::utils;
use crate::utils::builder::page::module::nav_bar::NavBar;
use crate::utils::template_engine::tera_template_engine::template_tera;
use crate::utils::transform::db_transform::{FromDbRow, get_collection_data, ToViewString};


pub fn select_template_engine(template_name: String, html: HashMap<&str, String>) -> String {
    match template_name.as_str() {
        "tera" => {
            let template_path = utils::env::get("TEMPLATE_ENGINE_BASE_PATH");
            template_tera(html, template_path)
        }
        _ => format!("No template \"{}\" selected", template_name),
    }
}

pub fn generate_html<T, U>(
    data: Vec<T>, // Source des données génériques
    template_name: &str,
    navbar: String,
    section: String,
    footer: String,
) -> String
where
    U: ToViewString + FromDbRow<T> + Serialize, // Ajoutez Serialize ici
{
    // Récupérer les données transformées depuis la source
    let data: Vec<U> = get_collection_data(&data);

    // Convertir les données en une chaîne de vue
    let data_view = data.to_view_string();

    // Créer une map HTML pour les paramètres du template
    let mut html_map: HashMap<&str, String> = HashMap::new();
    html_map.insert("html_navbar",  navbar);
    html_map.insert("html_section", section);
    html_map.insert("html_footer",  footer);
    html_map.insert("test",         data_view);

    // Sélectionner et exécuter le moteur de template
    select_template_engine(template_name.to_string(), html_map)
}




pub fn html_navbar()->String{
    let navbar = NavBar::new("MainNav".to_string(), Some("Événements".to_string()), None);
    navbar.to_html()
}

pub fn html_section()->String{
    "<div>SECTION</div>".to_string()
}

pub fn html_footer()->String{

    "<div>FOOTER</div>".to_string()
}

pub fn html_error()->String{
    "<div>ERROR</div>".to_string()
}

