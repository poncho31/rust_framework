use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct Display {
    pub content_count          : u32,
    pub max_element_horizontal : u32,
    pub space_between          : u32,
}
