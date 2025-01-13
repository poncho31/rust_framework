use actix_web::web;
use serde_derive::Serialize;
use crate::config::route_config::{get_web_routes, RouteInfoDisplay};
use crate::database::DbPool;
use crate::models::event_model::Event;
use crate::repository::event_repository;
use crate::utils::builder::page_builder::form::{Form, FormField, FormFieldType, IntoSelectOption, SelectOption};
use crate::utils::builder::page_builder::list::List;
use crate::utils::builder::page_builder::navbar::{NavBar, NavBarData, NavBarMetadata};
use crate::utils::builder::page_builder::section::{DataType, Section, SectionData, SectionDebug};
use crate::utils::builder::page_builder::table::Table;

#[derive(Serialize)]
pub struct PageBuilder {
    navbar: Option<NavBar>,
    section: Option<Section>,
}

impl PageBuilder {
    // Création d'une nouvelle instance de PageBuilder
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
            // NAVBAR
            navbar: Some(NavBar {
                // META
                meta_data: NavBarMetadata {
                    file_name: navbar_file_name.to_string(),
                    raw_data : NavBarData {
                        nav_title       : nav_title.to_string(),
                        page_title      : nav_page_title.to_string(),
                        drop_down_menu  : nav_drop_down_menu.clone(),
                        shortcut_menu   : nav_shortcut_menu.clone(),
                    },
                },
                // DATA
                data: NavBarData {
                    nav_title       : nav_title.to_string(),
                    page_title      : nav_page_title.to_string(),
                    drop_down_menu  : nav_drop_down_menu,
                    shortcut_menu   : nav_shortcut_menu,
                },
            }),
            // SECTION
            section: Some(Section {
                // META
                meta_data : SectionDebug {
                    file_name : section_file_name.to_string(),
                    raw_data  : SectionData {
                        title    : section_title.to_string(),
                        contents : section_contents.clone(),
                    },
                },
                // DATA
                data: SectionData {
                    title    : section_title.to_string(),
                    contents : section_contents,
                },
            }),
        }
    }


    // Modèle de base pour une page
    pub fn base_model(
        nav_title           : &str,
        nav_page_title      : &str,
        nav_drop_down_menu  : Option<Vec<RouteInfoDisplay>>,
        nav_shortcut_menu   : Option<Vec<RouteInfoDisplay>>,
        section_title       : &str,
        section_contents     : Vec<DataType>,
    ) -> Self {
        Self::new(
            // NAVBAR
            "navbar_tera.html",
            nav_title,
            nav_page_title,
            nav_drop_down_menu,
            nav_shortcut_menu,
            // SECTION
            "section_tera.html",
            section_title,
            vec![section_contents],
        )
    }
}


// Exemple d'utilisation de PageBuilder
pub fn page_builder_exemple(pool: web::Data<DbPool>) -> PageBuilder {
    let events: Vec<Event> = event_repository::paginate_events(pool.to_owned(), None, Some(100));
    let list_data: Vec<SelectOption> = events.to_select_option();

    let all_events = event_repository::paginate_events(pool, None, Some(100));


    let section_display_data =
        Form::create(
            vec![
                // INPUT TEXT
                FormField::new(
                    "Name",
                    "",
                    "section_name",
                    FormFieldType::Text{},
                    true,
                    Some("Section name")
                ),
                // INPUT TEXT long text
                FormField::new(
                    "TEST",
                    "TEST Lorem ipsum dolor sit amet, consectetur adipisicing elit. Aliquid autem dolorum facere libero molestiae necessitatibus quaerat quasi rem sed vitae! Incidunt molestias quo quod? Id iure odio possimus soluta veritatis!",
                    "section_name",
                    FormFieldType::Text{},
                    true,
                    Some("Section name")
                ),
                // INPUT DATE
                FormField::new(
                    "Date",
                    "",
                    "section_date",
                    FormFieldType::Date{},
                    true,
                    Some("Section date")
                ),
                // INPUT NUMBER
                FormField::new(
                    "Number",
                    "",
                    "section_number",
                    FormFieldType::Number{},
                    true,
                    Some("Section number")
                ),
                // SELECT option raw
                FormField::new(
                    "Section",
                    "",
                    "list_section",
                    FormFieldType::Select {
                        options: vec![
                            SelectOption {
                                name: "List".to_string(),
                                value: "list".to_string(),
                                selected: true,
                                disabled: false,
                            },
                            SelectOption {
                                name: "Table".to_string(),
                                value: "table".to_string(),
                                selected: false,
                                disabled: false,
                            },
                        ],
                        multiple: false,
                        debug   : false,
                    },
                    true,
                    None,
                ),

                // SELECT option from IntoSelectOption
                FormField::new(
                    "Section",
                    "",
                    "list_section",
                    FormFieldType::Select {
                        options: list_data,
                        multiple: false,
                        debug   : false,
                    },
                    true,
                    None,
                ),

                // TEXTAREA
                FormField::new(
                    "Textarea",
                    "",
                    "section_textarea",
                    FormFieldType::TextArea{},
                    true,
                    Some("Section textarea")
                ),

            ],
            "action".to_string(),
            "post".to_string(),
            "Envoyer".to_string()
        );


    // Construction de l'objet PageBuilder
    PageBuilder::base_model(
        // NAVBAR
        "Rust framework",
        "Page builder",
        Some(get_web_routes(Some("get"))),
        Some(get_web_routes(Some("get"))),
        // SECTION
        "Creation d'une page",
        vec![
            // Formulaire de création
            DataType::Form(section_display_data.clone()),
            DataType::Form(section_display_data.clone()),
            DataType::Form(section_display_data.clone()),
            DataType::Form(section_display_data.clone()),
            DataType::Form(section_display_data),
            // DataType::Table(Table::create(all_events))
        ],
    )
}
