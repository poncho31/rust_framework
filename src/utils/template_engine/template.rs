use std::collections::HashMap;
use serde::Serialize;
use serde_json::{to_value, Value};
use crate::utils;
use crate::utils::builder::page_builder::page_builder::PageBuilder;
use crate::utils::template_engine::debug_template_engine::debug_template_engine;
use crate::utils::template_engine::tera_template_engine::template_tera;
use crate::utils::transform::db_transform::{FromDbRow, get_collection_data, ToViewString};


pub fn select_template_engine(template_name: String, html: HashMap<&str, Value>) -> String {
    let template_path = utils::env::get("TEMPLATE_ENGINE_BASE_PATH");
    match template_name.as_str() {
        "tera" => {
            template_tera(html, template_path)
        }
        _ => format!("No template \"{}\" selected", template_name),
    }
}

pub fn generate_html<T, U>(
    template_name: &str,
    data: Vec<T>, // Source des données génériques
    page_builder: PageBuilder
) -> String
where
    U: ToViewString + FromDbRow<T> + Serialize, // Ajoutez Serialize ici
{
    // Créer une map HTML pour les paramètres du template
    let mut html_map: HashMap<&str, Value> = HashMap::new();
    html_map.insert("page_builder", to_value(&page_builder).unwrap());

    html_map.insert("debug_template_engine", debug_template_engine(to_value(&html_map)));
    html_map.insert("content", debug_template_engine(to_value(page_builder)));


    // Sélectionner et exécuter le moteur de template
    select_template_engine(template_name.to_string(), html_map)
}


pub fn html_error(error: String) ->String{
    format!("<div>ERROR : <br> {}</div>", error)
}

