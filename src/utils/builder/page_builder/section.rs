use enum_macro::EnumMacro;
use serde::Deserialize;
use serde_derive::Serialize;
use crate::utils::builder::page_builder::form::Form;
use crate::utils::builder::page_builder::list::List;
use crate::utils::builder::page_builder::table::Table;
use crate::utils::common::generate_random_string;

use super::form::SelectOption;
use super::widget::Widget;

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct Section {
    pub id        : String,
    pub file_name : String,
    pub title     : String,
    pub contents  : Vec<Vec<DataType>>,
}

impl Section {
    pub fn new(section_file_name: String, section_title: String, section_contents: Vec<Vec<DataType>>) -> Section {
        Section {
            id: format!("id_section_{}", generate_random_string(10)).parse().unwrap(),
            file_name: section_file_name.to_string(),
            title: section_title.to_string(),
            contents: section_contents,
        }
    }
}

#[derive(Serialize, Clone, EnumMacro, Deserialize, Debug)]
pub enum DataType {
    Table(Table),
    List(List),
    Form(Form),
    Widget(Widget),
}


impl DataType {
    /// Retourne un `Vec<SelectOption>` basé sur les noms des variantes de l'enum
    pub fn to_select_option() -> Vec<SelectOption> {
        // Utilise to_vec() généré par la macro EnumMacro pour obtenir les noms des variantes
        DataType::to_vec()
            .into_iter()
            .map(|variant_name| SelectOption {
                name: variant_name.to_string(),
                value: variant_name.to_string(),
                selected: false,
                disabled: false,
            })
            .collect()
    }
}