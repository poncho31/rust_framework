use crate::utils::builder::page::module::nav_bar::NavBar;
use crate::view::event_table::EventTable; // Utilisation d'EventTable
use crate::utils::builder::page::module::section::Section;
use crate::utils::builder::page::page_builder::PageBuilder;

pub struct EventManager {
    navbar: NavBar,
    section: Section<EventTable>, // Section contenant EventTable
}

impl EventManager {
    pub fn new(navbar: NavBar, event_table: EventTable) -> Self {
        // Création de la section pour afficher le tableau des événements
        let section = Section::new(
            "Liste des événements".to_string(),
            Some("Liste des événements disponibles".to_string()),
            Some(event_table), // EventTable intégré dans la section
        );

        EventManager { navbar, section }
    }

    pub fn render_page(&self) -> String {
        let page = PageBuilder::new(
            "Liste des événements et ajout d'un événement".to_string(),
            self.navbar.clone(), // Utilisation de `.clone()` sur `NavBar`
            self.section.to_html(),
        )
            .with_modal(include_str!("../view/templates/modal_add_event.html").to_string())
            .with_script(include_str!("../view/templates/event_manager_script.html").to_string());

        page.render()
    }

}
