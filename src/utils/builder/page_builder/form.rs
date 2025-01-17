use diesel::sql_types::Bool;
use serde::Serialize;
use crate::utils::common::generate_random_string;

#[derive(Serialize, Clone, Debug)]
pub struct Form {
    pub id                 : String,
    pub title              : String,
    pub fields             : Vec<FormField>,
    pub action             : String,
    pub method             : String,
    pub submit_label       : String,
    pub template_file_path : String,
    pub css_file_path      : Option<String>,
}

impl Form {
    pub fn new(title: String,action: String, method: String, submit_label: String,fields: Vec<FormField>) -> Self {
        Self {
            id: format!("id_form_{}", generate_random_string(10)),
            title,
            fields,
            action,
            method,
            submit_label,
            template_file_path: "template/tera/form_tera.html".to_string(),
            css_file_path: Some("template".to_string()),
        }
    }

    // Génération d'un formulaire à partir d'une liste de champs
    pub fn create(title : String , fields: Vec<FormField>, action: String, method: String, submit_label : String) -> Self {
        Self::new(title,action, method, submit_label,fields)
    }
}

#[derive(Serialize, Clone, Debug)]
pub enum FormFieldType {
    Text{},
    Number{},
    Date{},
    Select {
        options  : Vec<SelectOption>,
        multiple : bool,
        debug    : bool,
    },
    TextArea{},
}

#[derive(Serialize, Clone, Debug)]
pub struct FormField {
    pub id: String,
    pub label: String,
    pub label_long: String,
    pub name: String,
    // pub disable : bool,
    // pub form    : String,
    pub field_type: FormFieldType,
    pub required: bool,
    pub placeholder: Option<String>,
}

impl FormField {
    pub fn new(
        label: &str,
        label_long: &str,
        name: &str,
        field_type: FormFieldType,
        required: bool,
        placeholder: Option<&str>,
    ) -> Self {
        Self {
            id: format!("id_field_form_{}", generate_random_string(10)),
            label : label.to_string(),
            label_long : label_long.to_string(),
            name : name.to_string(),
            field_type,
            required,
            placeholder: placeholder.map(|p| p.to_string()),
        }
    }
}


#[derive(Serialize, Clone, Debug)]
pub struct SelectOption {
    pub name     : String,
    pub value    : String,
    pub selected : bool,
    pub disabled : bool,
}

pub trait IntoSelectOption {
    fn to_select_option(&self) -> Vec<SelectOption>;
}

impl SelectOption {
    pub fn create<T: IntoSelectOption>(data: Vec<T>) -> Vec<SelectOption> {
        data.into_iter()
            .flat_map(|item| item.to_select_option()) // Combine les vecteurs retournés par `to_select_option`
            .collect()
    }
}
