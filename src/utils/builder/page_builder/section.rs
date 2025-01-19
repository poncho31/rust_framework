use enum_own::EnumOwn;
use serde_derive::Serialize;
use crate::utils::builder::page_builder::form::Form;
use crate::utils::builder::page_builder::list::List;
use crate::utils::builder::page_builder::table::Table;

use super::form::{IntoSelectOption, SelectOption};

#[derive(Serialize, Clone)]
pub struct Section {
    pub file_name : String,
    
    pub title     : String,
    pub contents  : Vec<Vec<DataType>>,
}

#[derive(Serialize, Clone, EnumOwn)]
pub enum DataType {
    Table(Table),
    List(List),
    Form(Form),
}


impl DataType {
    /// Retourne un `Vec<SelectOption>` basé sur les noms des variantes de l'enum
    pub fn to_select_option() -> Vec<SelectOption> {
        // Utilise to_vec() généré par la macro EnumOwn pour obtenir les noms des variantes
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