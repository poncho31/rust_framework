use serde_derive::Serialize;
use crate::utils::builder::page_builder::form::Form;
use crate::utils::builder::page_builder::list::List;
use crate::utils::builder::page_builder::table::Table;

#[derive(Serialize)]
pub struct Section {
    pub meta_data  : SectionDebug,
    pub data       : SectionData,
}
#[derive(Serialize, Clone)]
pub struct SectionDebug {
    pub file_name : String,
    pub raw_data  : SectionData,
}


#[derive(Serialize, Clone)]
pub struct SectionData {
    pub title    : String,
    pub contents : Vec<Vec<DataType>>,
    pub display : u32,
}

#[derive(Serialize, Clone)]
pub enum DataType {
    Table(Table),
    List(List),
    Form(Form),
}
