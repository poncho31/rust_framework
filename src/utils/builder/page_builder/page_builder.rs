use actix_web::web;
use serde_derive::Serialize;
use crate::config::route_config::{get_web_routes, RouteInfoDisplay};
use crate::database::DbPool;
use crate::models::event_model::Event;
use crate::repository::event_repository;
use crate::utils::builder::page_builder::form::{Form, FormField, FormFieldType, IntoSelectOption, SelectOption};
use crate::utils::builder::page_builder::section::{DataType, Section};
use crate::utils::builder::page_builder::navbar::NavBar;
use crate::utils::builder::page_builder::display::Display;
use rand::Rng;




#[derive(Serialize)]
pub struct PageBuilder {
    navbar  : Option<NavBar>,
    section : Option<Section>,
    display : Option<Display>,
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

        display_max_element_horizontal : u32,
        display_space_between          : u32,
    ) -> Self {
        Self {
            // NAVBAR
            navbar: Some(NavBar {
                file_name       : navbar_file_name.to_string(),
                nav_title       : nav_title.to_string(),
                page_title      : nav_page_title.to_string(),
                drop_down_menu  : nav_drop_down_menu,
                shortcut_menu   : nav_shortcut_menu,
            }),
            
            // SECTION
            section: Some(Section {
                file_name : section_file_name.to_string(),
                title    : section_title.to_string(),
                contents : section_contents.clone(),
            }),
            
            // DISPLAY
            display: Some(Display{
                content_count          : section_contents.iter().flat_map(|inner_vec| inner_vec).count() as u32,
                max_element_horizontal : display_max_element_horizontal,
                space_between          : display_space_between,
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
        section_contents    : Vec<DataType>
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
            3,
            1
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
            DataType::Form(page_builder_form(false)),
        ]
    )
}

pub fn page_builder_form(debug: bool) -> Form {
    let fields = vec![

        // Page name
        FormField::new_simple(
            /*  name       : */ "page_name",
            /*  field_type : */ FormFieldType::Text {},
            /*  required   : */ true,
            /*  placeholder: */ Some("Nom de la page à créer"),
        ),

        // Description simple
        FormField::new_simple(
            /*  name       : */ "random_textarea",
            /*  field_type : */ FormFieldType::TextArea {},
            /*  required   : */ false,
           /*  placeholder: */  Some("Description rapide de la page"),
        ),

        // Fichier
        FormField::new_simple(
            /*  name       : */ "file_name",
            /*  field_type : */ FormFieldType::File {},
            /*  required   : */ true,
            /*  placeholder: */ Some("Sélectionnez un / plusieurs fichiers qui composeront votre page"),
        ),

        // Type d'affichage pour le fichier
        FormField::new_simple(
             /*  name       : */  "display_type_name",
             /*  field_type : */ FormFieldType::Select {
                                    options  : DataType::to_select_option(),
                                    multiple : false,
                                    debug,
                                },
            /*  required   : */ true,
            /*  placeholder: */ Some("Type d'affichage pour le fichier"),
        ),
    ];

    Form::create(
        "Formulaire de création de page".to_string(),
        fields,
        "/".to_string(),
        "post".to_string(),
        "Envoyer le formulaire".to_string(),
    )
}




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