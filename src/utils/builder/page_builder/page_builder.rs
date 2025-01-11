use std::collections::HashMap;
use actix_web::web;
use serde_derive::Serialize;
use crate::config::route_config::{get_web_routes, RouteInfoDisplay};
use crate::utils::builder::page_builder::navbar::{NavBar, NavBarData, NavBarMetadata};
use crate::utils::builder::page_builder::section::{DataType, Section, SectionData, SectionDebug};

#[derive(Serialize)]
pub struct PageBuilder {
    navbar: Option<NavBar>,
    section: Option<Section>,
}

impl PageBuilder {
    /// Création d'une nouvelle instance de PageBuilder
    pub fn new(
        navbar_file_name    : &str,
        nav_title           : &str,
        nav_page_title      : &str,
        nav_drop_down_menu  : Option<Vec<RouteInfoDisplay>>,
        nav_shortcut_menu   : Option<Vec<RouteInfoDisplay>>,

        section_file_name   : &str,
        section_title       : &str,
        section_contents     : Vec<Vec<DataType>>,
    ) -> Self {
        Self {
            /// NAVBAR
            navbar: Some(NavBar {
                meta_data: NavBarMetadata {
                    file_name: navbar_file_name.to_string(),
                    raw_data : NavBarData {
                        nav_title       : nav_title.to_string(),
                        page_title      : nav_page_title.to_string(),
                        drop_down_menu  : nav_drop_down_menu.clone(),
                        shortcut_menu   : nav_shortcut_menu.clone(),
                    },
                },
                data: NavBarData {
                    nav_title       : nav_title.to_string(),
                    page_title      : nav_page_title.to_string(),
                    drop_down_menu  : nav_drop_down_menu,
                    shortcut_menu   : nav_shortcut_menu,
                },
            }),
            /// SECTION
            section: Some(Section {
                meta_data : SectionDebug {
                    file_name : section_file_name.to_string(),
                    raw_data  : SectionData {
                        title    : section_title.to_string(),
                        contents : section_contents.clone(),
                    },
                },
                data: SectionData {
                    title    : section_title.to_string(),
                    contents : section_contents,
                },
            }),
        }
    }


    /// Modèle de base pour une page
    pub fn base_model(
        nav_title           : &str,
        nav_page_title      : &str,
        nav_drop_down_menu  : Option<Vec<RouteInfoDisplay>>,
        nav_shortcut_menu   : Option<Vec<RouteInfoDisplay>>,
        section_title       : &str,
        section_contents     : Vec<DataType>,
    ) -> Self {
        Self::new(
            /// NAVBAR
            "navbar_tera.html",
            nav_title,
            nav_page_title,
            nav_drop_down_menu,
            nav_shortcut_menu,
            /// SECTION
            "section_tera.html",
            section_title,
            vec![section_contents],
        )
    }
}


/// Exemple d'utilisation de PageBuilder
pub fn example() -> PageBuilder {
    PageBuilder::new(
        /// NAVBAR
        "navbar_tera.html",
        "Custom Event Manager",
        "nav_page_title",
        Some(get_web_routes(Some("get"))),
        Some(get_web_routes(Some("get"))),
        /// SECTION
        "section_tera.html",
        "Welcome Section",
        vec![],
    )
}
