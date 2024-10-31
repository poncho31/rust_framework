use crate::utils::builder::page::module::nav_bar::NavBar;

pub struct PageBuilder {
    title: String,
    navbar: NavBar,
    main_content: String, // Changement de type pour accepter une String au lieu de Section
    modal_content: Option<String>,
    script_content: Option<String>,
}

impl PageBuilder {
    pub fn new(title: String, navbar: NavBar, main_content: String) -> Self {
        PageBuilder {
            title,
            navbar,
            main_content,
            modal_content: None,
            script_content: None,
        }
    }

    pub fn with_modal(mut self, modal_content: String) -> Self {
        self.modal_content = Some(modal_content);
        self
    }

    pub fn with_script(mut self, script_content: String) -> Self {
        self.script_content = Some(script_content);
        self
    }

    pub fn render(&self) -> String {
        format!(
            r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>{}</title>
                <link rel="stylesheet" href="/resources/css/packages/bulma.min.css">
            </head>
            <body>
                <nav>{}</nav>
                <section class="section">
                    <div class="container">
                        {}
                    </div>
                </section>
                {}
                <script src="/resources/js/app.js"></script>
                {}
            </body>
            </html>
            "#,
            self.title,
            self.navbar.to_html(),
            self.main_content,
            self.modal_content.clone().unwrap_or_default(),
            self.script_content.clone().unwrap_or_default()
        )
    }
}
