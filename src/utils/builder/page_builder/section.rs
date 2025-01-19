use serde_derive::Serialize;
use crate::utils::builder::page_builder::form::Form;
use crate::utils::builder::page_builder::list::List;
use crate::utils::builder::page_builder::table::Table;

#[derive(Serialize, Clone)]
pub struct Section {
    pub file_name : String,
    
    pub title     : String,
    pub contents  : Vec<Vec<DataType>>,
}

#[derive(Serialize, Clone)]
pub enum DataType {
    Table(Table),
    List(List),
    Form(Form),
}

