use crate::utils::builder::page::module::nav_bar::NavBar;

pub struct PageBuilder {
    title: String,
    nav_bar: NavBar,
    section_content: String, // Changement de type pour accepter une String au lieu de Section
}

impl PageBuilder {
    pub fn new(title: String, nav_bar: NavBar, section_content: String) -> Self {
        PageBuilder {
            title,
            nav_bar,
            section_content,
        }
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
                <link rel="stylesheet" href="/resources/css/app.css">
            </head>
            <body>
                <nav>{}</nav>
                <section class="section">
                    <div class="container">
                        {}
                    </div>
                </section>
                <script src="/resources/js/app.js"></script>
            </body>
            </html>
            "#,
            self.title,
            self.nav_bar.to_html(),
            self.section_content,
        )
    }
}
