#[derive(Clone)]
pub struct Button {
    id: String,
    name: String,
    url: Option<String>,
    button_tag: ButtonTag,
    button_type: ButtonType,
}

// Assurez-vous que les enums dérivent également `Clone`
#[derive(Clone, Default)]
enum ButtonType {
    #[default]
    Default,
    Danger,
    Info,
    Success,
    Warning,
}

#[derive(Clone, Default)]
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

    pub fn to_html(&self) -> String {
        let tag = match self.button_tag {
            ButtonTag::A => "a",
            ButtonTag::Button => "button",
            ButtonTag::Input => "input",
        };

        format!(
            r#"<{tag} id="{id}" class="btn btn-{type}">{name}</{tag}>"#,
            tag = tag,
            id = self.id,
            type = match self.button_type {
                ButtonType::Default => "default",
                ButtonType::Danger => "danger",
                ButtonType::Info => "info",
                ButtonType::Success => "success",
                ButtonType::Warning => "warning",
            },
            name = self.name,
        )
    }
}
