use actix_web::web;
use serde_derive::Serialize;
use crate::config::route_config::{get_web_routes, RouteInfo, RouteInfoDisplay};
use crate::controllers::event_controller::list_events;
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
        section_contents    : Vec<Vec<DataType>>,
        section_display     : u32
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
                        display  : section_display.clone(),
                    },
                },
                // DATA
                data: SectionData {
                    title    : section_title.to_string(),
                    contents : section_contents,
                    display  : section_display,
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
        section_contents    : Vec<DataType>,
        section_display     : u32,
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
            section_display
        )
    }
}


// Exemple d'utilisation de PageBuilder
pub fn page_builder_exemple(pool: web::Data<DbPool>) -> PageBuilder {
    let events: Vec<Event> = event_repository::paginate_events(pool.to_owned(), None, Some(100));
    let list_data: Vec<SelectOption> = events.to_select_option();

    let all_events = event_repository::paginate_events(pool, None, Some(100));



    // Construction de l'objet PageBuilder
    PageBuilder::base_model(
        // NAVBAR
        "Rust framework",
        "",
        Some(get_web_routes(Some("get"))),
        Some(generate_random_shortcut()),
        // SECTION
        "Creation d'une page Web",
        vec![
            // Formulaire de création
            DataType::Form(generate_random_form(false)),
            DataType::Form(generate_random_form(false)),
            DataType::Form(generate_random_form(false)),
            DataType::Form(generate_random_form(false)),

            // Table avec éléments
            DataType::Table(Table::create("Table",all_events))
        ],
        3
    )
}


use rand::Rng;


pub fn generate_random_shortcut()->Vec<RouteInfoDisplay>{
     vec![
        RouteInfoDisplay {
            name: "\
            <div style='display: flex; align-items: center;'>
                <img
                    src=\"https://upload.wikimedia.org/wikipedia/commons/d/d5/Rust_programming_language_black_logo.svg\"
                    alt=\"Rust Logo\"
                    style=\"width: 24px; height: 24px; margin-right: 8px;\"
                >
            </div>".to_string(),
            uri: "https://www.rust-lang.org/".to_string(),
            method: "get".to_string(),
        },
        RouteInfoDisplay {
            name: "\
            <div style='display: flex; align-items: center;'>
                <img
                    src=\"https://cdn-icons-png.flaticon.com/512/2111/2111628.png\"
                    alt=\"GitHub Icon\"
                    style=\"width: 24px; height: 24px; margin-right: 8px;\"
                >
            </div>".to_string(),
            uri: "https://github.com/".to_string(),
            method: "get".to_string(),
        },
        RouteInfoDisplay {
            name: "\
            <div style='display: flex; align-items: center;'>
                <img
                    src=\"https://cdn-icons-png.flaticon.com/512/732/732200.png\"
                    alt=\"HTML5 Icon\"
                    style=\"width: 24px; height: 24px; margin-right: 8px;\"
                >
            </div>".to_string(),
            uri: "https://developer.mozilla.org/en-US/docs/Web/HTML".to_string(),
            method: "get".to_string(),
        },
        RouteInfoDisplay {
            name: "\
            <div style='display: flex; align-items: center;'>
                <img
                    src=\"https://cdn-icons-png.flaticon.com/512/5968/5968705.png\"
                    alt=\"CSS Icon\"
                    style=\"width: 24px; height: 24px; margin-right: 8px;\"
                >
            </div>".to_string(),
            uri: "https://developer.mozilla.org/en-US/docs/Web/CSS".to_string(),
            method: "get".to_string(),
        },
        RouteInfoDisplay {
            name: "\
            <div style='display: flex; align-items: center;'>
                <img
                    src=\"https://cdn-icons-png.flaticon.com/512/226/226777.png\"
                    alt=\"JavaScript Icon\"
                    style=\"width: 24px; height: 24px; margin-right: 8px;\"
                >
            </div>".to_string(),
            uri: "https://developer.mozilla.org/en-US/docs/Web/JavaScript".to_string(),
            method: "get".to_string(),
        },
        RouteInfoDisplay {
            name: "\
            <div style='display: flex; align-items: center;'>
                <img
                    src=\"https://cdn-icons-png.flaticon.com/512/919/919827.png\"
                    alt=\"Node.js Icon\"
                    style=\"width: 24px; height: 24px; margin-right: 8px;\"
                >
            </div>".to_string(),
            uri: "https://nodejs.org/".to_string(),
            method: "get".to_string(),
        },
        RouteInfoDisplay {
            name: "Evénements".to_string(),
            uri: "/".to_string(),
            method: "get".to_string(),
        }
    ]
}
pub fn generate_random_form(debug: bool) -> Form {
    let mut rng = rand::thread_rng();

    let fields = vec![
        // Random text field
        FormField::new(
            "Random Name",
            "",
            "random_name",
            FormFieldType::Text {},
            rng.gen_bool(0.5),
            Some("A randomly generated name"),
        ),
        // Random long text field
        FormField::new(
            "Random Description",
            "Lorem ipsum dolor sit amet, consectetur adipisicing elit.",
            "random_description",
            FormFieldType::Text {},
            rng.gen_bool(0.5),
            Some("A randomly generated description"),
        ),
        // Random date field
        FormField::new(
            "Random Date",
            "",
            "random_date",
            FormFieldType::Date {},
            rng.gen_bool(0.5),
            Some("A randomly generated date"),
        ),
        // Random number field
        FormField::new(
            "Random Number",
            &rng.gen_range(1..=100).to_string(),
            "random_number",
            FormFieldType::Number {},
            rng.gen_bool(0.5),
            Some("A randomly generated number"),
        ),
        // Random select field
        FormField::new(
            "Random Selection",
            "",
            "random_selection",
            FormFieldType::Select {
                options: vec![
                    SelectOption {
                        name: "Option 1".to_string(),
                        value: "option1".to_string(),
                        selected: rng.gen_bool(0.5),
                        disabled: false,
                    },
                    SelectOption {
                        name: "Option 2".to_string(),
                        value: "option2".to_string(),
                        selected: rng.gen_bool(0.5),
                        disabled: false,
                    },
                ],
                multiple: false,
                debug,
            },
            rng.gen_bool(0.5),
            None,
        ),
        // Random textarea field
        FormField::new(
            "Random Textarea",
            "",
            "random_textarea",
            FormFieldType::TextArea {},
            rng.gen_bool(0.5),
            Some("A randomly generated textarea"),
        ),
    ];

    Form::create(
        "Random Form".to_string(),
        fields,
        "random_action".to_string(),
        "post".to_string(),
        "Submit".to_string(),
    )
}

