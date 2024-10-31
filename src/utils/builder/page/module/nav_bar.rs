use crate::utils::builder::page::module::button::Button;

#[derive(Clone)]
pub struct NavBar {
    id: String,
    title: String,
    page_title: Option<String>,
    buttons: Option<Vec<Button>>,
    navbar_type: NavBarType,
}

#[derive(Clone, Default)]
enum NavBarType {
    #[default]
    Default,
}

impl NavBar {
    pub fn new(
        title: String,
        page_title: Option<String>,
        buttons: Option<Vec<Button>>,
    ) -> Self {
        NavBar {
            id: format!("navbar_{}", &title),
            title,
            page_title,
            buttons,
            navbar_type: NavBarType::default(),
        }
    }

    pub fn to_html(&self) -> String {
        // Définir la classe de la navbar selon son type
        let class = match self.navbar_type {
            NavBarType::Default => "navbar-default",
        };

        // Générer le HTML des boutons s'il y en a
        let buttons_html = if let Some(buttons) = &self.buttons {
            buttons.iter().map(|button| button.to_html()).collect::<Vec<_>>().join("\n")
        } else {
            String::new()
        };

        // Générer le code HTML complet de la navbar
        format!(
            r#"
            <nav id="{id}" class="navbar {class}">
                <div class="navbar-header">
                    <span class="navbar-brand">{title}</span>
                    {page_title}
                </div>
                <div class="navbar-buttons">
                    {buttons}
                </div>
            </nav>
            "#,
            id         = self.id,
            class      = class,
            title      = self.title,
            page_title = self.page_title.clone().unwrap_or_default(),
            buttons    = buttons_html
        )
    }
}
