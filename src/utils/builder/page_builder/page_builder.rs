use std::collections::HashMap;
use serde_derive::Serialize;
use crate::utils::builder::page_builder::navbar::{NavBar, NavBarData, NavBarMetadata};
use crate::utils::builder::page_builder::section::{Section, SectionData, SectionDebug};

#[derive(Serialize)]
pub struct PageBuilder {
    navbar  : Option<NavBar>,
    section : Option<Section>,
}



impl PageBuilder {
    pub fn new(
        navbar_file_path: String,
        nav_title: String,
        nav_page_title: String,
        nav_drop_down_menu: Option<Vec<(String, String)>>,
        section_file_path: String,
        section_title: String,
        section_content: String,
    ) -> Self {
        let nav_data = NavBarData {
            title          : nav_title.to_string(),
            page_title     :  nav_page_title,
            drop_down_menu : nav_drop_down_menu.clone(),
        };
        let navbar = NavBar {
            meta_data: NavBarMetadata {
                file_path : navbar_file_path.to_string(),
                raw_data  : nav_data.clone(),
            },
            data: nav_data,
        };


        let section_data = SectionData {
            title   : section_title.to_string(),
            content : section_content.to_string(),
        };
        let section = Section {
            debug_data: SectionDebug {
                file_path : section_file_path.to_string(),
                raw_data  : section_data.clone(),
            },
            template_data : section_data,
        };

        PageBuilder {
            navbar  : Some(navbar),
            section : Some(section),
        }
    }

    pub fn base_model(
        nav_title: &str,
        nav_page_title: &str,
        nav_drop_down_menu: Option<Vec<(String, String)>>,
        section_title: &str,
        section_content: &str,
    ) ->PageBuilder{
        PageBuilder::new(
            "template/tera/navbar_tera.html".to_string(),
            nav_title.to_string(),
            nav_page_title.to_string(),
            nav_drop_down_menu,
            "templates/tera/section.html".to_string(),
            section_title.to_string(),
            section_content.to_string(),
        )
    }
}

pub fn example()->PageBuilder {
    let page_builder = PageBuilder::new(
        "templates/tera/navbar.html".to_string(),
        "Custom Event Manager".to_string(),
        "nav_page_title".to_string(),
        Some(vec![
            ("Utilisateurs".to_string(), "/users".to_string()),
            ("Déconnexion".to_string(), "/users/logout".to_string()),
        ]),
        "templates/tera/section.html".to_string(),
        "Welcome Section".to_string(),
        "This is the main content of the page.".to_string(),
    );

    if let Some(navbar) = &page_builder.navbar {
        println!("Navbar Debug Data: {{ title: {}, page_title: {:?}, drop_down_menu: {:?} }}",
                 navbar.meta_data.raw_data.title,
                 navbar.meta_data.raw_data.page_title,
                 navbar.meta_data.raw_data.drop_down_menu
        );
    }

    if let Some(section) = &page_builder.section {
        println!("Section Debug Data: {{ title: {}, content: {} }}",
                 section.debug_data.raw_data.title,
                 section.debug_data.raw_data.content
        );
    }

    page_builder
}
