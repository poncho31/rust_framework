use serde_derive::Serialize;
use crate::utils::builder::page_builder::table::Table;

#[derive(Serialize)]
pub struct Section {
    pub meta_data  : SectionDebug,
    pub data       : SectionData,
}
#[derive(Serialize, Clone)]
pub struct SectionDebug {
    pub file_path : String,
    pub raw_data  : SectionData,
}


#[derive(Serialize, Clone)]
pub struct SectionData {
    pub title: String,
    pub content: Vec<DataType>, // Contient des tables, mais pourrait être étendu
}

#[derive(Serialize, Clone)]
pub enum DataType {
    Table(Table),
}
