use serde::{Deserialize, Serialize};
use crate::utils::common::{formatted_date, generate_id};

use super::html::{Html, Tag};

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct Widget {
    pub id                  : String,
    pub class               : String,
    pub title               : String,
    pub html_elements       : Vec<String>,
    pub template_file_path  : String,
    pub css_file_path       : Option<String>,
}

impl Widget {
    pub fn new(title: String, html_elements: Vec<String>) -> Self {
        Self {
            id                 : generate_id("id_widget"),
            class              : "widget".to_string(),
            title,
            html_elements,
            template_file_path : "template/tera/widget/widget_panel_tera.html".to_string(),
            css_file_path      : Some("template".to_string()),
        }
    }

    // Fonction générique pour construire une Widget à partir d'une liste de données
    pub fn create<T: IntoHtmlWidget>(title: &str,data: Vec<T>) -> Self {

        let html_elements : Vec<String>      = T::html_elements();

        Self::new(title.to_string(),html_elements)
    }
}

pub trait IntoHtmlWidget {
    fn html_elements() -> Vec<String>;
    fn to_row(&self) -> Vec<String>;
}

pub fn drop_zone_widget() ->Widget{

    
    Widget {
        id: format!("id_Widget_dropzone_{}",  formatted_date("%Y_%m_%d_%H_%M_%S")).parse().unwrap(),
        class: format!("class_Widget_dropzone_{}",  formatted_date("%Y_%m_%d_%H_%M_%S")).parse().unwrap(),
        title: "Drop Zone".to_string(),
        html_elements: vec!("test".to_string()),
        template_file_path : "template/tera/widget/widget_dropzone_tera.html".to_string(),
        css_file_path      : Some("template".to_string()),
    }

}

