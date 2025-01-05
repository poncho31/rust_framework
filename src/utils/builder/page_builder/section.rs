use serde_derive::Serialize;

#[derive(Serialize)]
pub struct Section {
    pub debug_data    : SectionDebug,
    pub template_data : SectionData,
}
#[derive(Serialize)]
pub struct SectionDebug {
    pub file_path : String,
    pub raw_data  : SectionData,
}

#[derive(Clone, Serialize)]
pub struct SectionData {
    pub title   : String,
    pub content : String,
}
