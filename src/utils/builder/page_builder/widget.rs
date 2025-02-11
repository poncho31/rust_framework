use serde::{Deserialize, Serialize};
use crate::utils::common::{formatted_date, generate_random_string};

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct Widget {
    pub id                  : String,
    pub title               : String,
    pub headers             : Vec<String>,
    pub rows                : Vec<Vec<String>>,
    pub template_file_path  : String,
    pub css_file_path       : Option<String>,
}

impl Widget {
    pub fn new(title: String, headers: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        Self {
            id: format!("id_Widget_{}",  generate_random_string(10)).parse().unwrap(),
            title,
            headers,
            rows,
            template_file_path : "template/tera/widget/widget_panel_tera.html".to_string(),
            css_file_path      : Some("template".to_string()),
        }
    }

    // Fonction générique pour construire une Widget à partir d'une liste de données
    pub fn create<T: IntoHtmlWidget>(title: &str,data: Vec<T>) -> Self {

        let headers : Vec<String>      = T::headers();
        let rows    : Vec<Vec<String>> = data.into_iter().map(|item| item.to_row()).collect();

        Self::new(title.to_string(),headers, rows)
    }
}

pub trait IntoHtmlWidget {
    fn headers() -> Vec<String>;
    fn to_row(&self) -> Vec<String>;
}

pub fn drop_zone_widget(headers : Vec<String>, rows : Vec<Vec<String>>) ->Widget{
    Widget {
        id: format!("id_Widget_dropzone_{}",  formatted_date("%Y_%m_%d_%H_%M_%S")).parse().unwrap(),
        title: "Drop Zone".to_string(),
        headers,
        rows,
        template_file_path : "template/tera/widget/widget_dropzone_tera.html".to_string(),
        css_file_path      : Some("template".to_string()),
    }
}
