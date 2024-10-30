
// BUTTONS
pub struct Button {
    id: String,
    name: String,
    url: Option<String>,
    button_tag: ButtonTag,
    button_type: ButtonType,
}
#[derive(Default)]
enum ButtonType {
    #[default]
    Default,
    Danger,
    Info,
    Success,
    Warning,
}

#[derive(Default)]
enum ButtonTag {
    A,
    #[default]
    Button,
    Input,
}

impl Button {
    pub fn new(name: String, url: Option<String>) -> Self {
        Button {
            id: format!("button_{}", name),
            name,
            url,
            button_tag: ButtonTag::default(),
            button_type: ButtonType::default(),
        }
    }
}
