use crate::utils::builder::page::module::nav_bar::NavBar;
use crate::view::event_table::EventTable; // Utilisation d'EventTable
use crate::utils::builder::page::module::section::Section;
use crate::utils::builder::page::page_builder::PageBuilder;

pub struct TestView {
    navbar: NavBar,
    section: Section<EventTable>, // Section contenant EventTable
}

impl TestView {
    pub fn new(navbar: NavBar, event_table: EventTable) -> Self {
        // Création de la section pour afficher le tableau des événements
        let section = Section::new(
            "Liste des événements".to_string(),
            Some("Liste des événements disponibles".to_string()),
            Some(event_table), // EventTable intégré dans la section
        );

        TestView { navbar, section }
    }

    pub fn render_page(&self) -> String {
        let page = PageBuilder::new(
            "Liste des événements et ajout d'un événement".to_string(),
            self.navbar.clone(), // Utilisation de `.clone()` sur `NavBar`
            self.section.to_html(),
        );

        page.render()
    }

}
