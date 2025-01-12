use serde::Serialize;
use crate::utils::common::generate_random_string;

#[derive(Serialize, Clone, Debug)]
pub struct Form {
    pub id: String,
    pub fields: Vec<FormField>,
    pub action: String,
    pub method: String,
    pub template_file_path: String,
    pub css_file_path: Option<String>,
}

impl Form {
    pub fn new(action: String, method: String, fields: Vec<FormField>) -> Self {
        Self {
            id: format!("id_form_{}", generate_random_string(10)),
            fields,
            action,
            method,
            template_file_path: "template/tera/form_tera.html".to_string(),
            css_file_path: Some("template".to_string()),
        }
    }

    // Génération d'un formulaire à partir d'une liste de champs
    pub fn create(fields: Vec<FormField>, action: String, method: String) -> Self {
        Self::new(action, method, fields)
    }
}

#[derive(Serialize, Clone, Debug)]
pub enum FormFieldType {
    Text{},
    Number{},
    Date{},
    Select {
        options: Vec<String>,
        multiple: bool
    },
    TextArea{},
}

#[derive(Serialize, Clone, Debug)]
pub struct FormField {
    pub id: String,
    pub label: String,
    pub name: String,
    pub field_type: FormFieldType,
    pub required: bool,
    pub placeholder: Option<String>,
}

impl FormField {
    pub fn new(
        label: &str,
        name: &str,
        field_type: FormFieldType,
        required: bool,
        placeholder: Option<&str>,
    ) -> Self {
        Self {
            id: format!("id_field_form_{}", generate_random_string(10)),
            label : label.to_string(),
            name : name.to_string(),
            field_type,
            required,
            placeholder: placeholder.map(|p| p.to_string()),
        }
    }

}
