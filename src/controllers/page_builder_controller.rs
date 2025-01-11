use actix_web::{HttpResponse, web};
use crate::config::route_config::get_web_routes;
use crate::database::DbPool;
use crate::utils::builder::page_builder::form::{Form, FormField, FormFieldType};
use crate::utils::builder::page_builder::page_builder::PageBuilder;
use crate::utils::builder::page_builder::section::DataType;
use crate::utils::template_engine::template::generate_html;


pub async fn page_builder_view(pool: web::Data<DbPool>) -> HttpResponse {

    let section_display_data =
        Form::create(
            vec![
                FormField::new(
                    "Section",
                    "list_section",
                    FormFieldType::Select {
                                options: vec!["List".to_string(), "Table".to_string()],
                                multiple: false,
                            },
                    true,
                    None
                ),
                FormField::new(
                    "Name",
                    "section_name",
                    FormFieldType::Text{},
                    true,
                    Some("Section name")
                ),

            ],
            "action".to_string(),
            "post".to_string(),
        );

    // Construction de l'objet PageBuilder
    let page_builder = PageBuilder::base_model(
        // NAVBAR
        "Rust framework",
        "Page builder",
        Some(get_web_routes(Some("get"))),
        Some(get_web_routes(Some("get"))),
        // SECTION
        "",
        vec![
            DataType::Form(section_display_data),
        ], // Injecte le tableau dans la section
    );

    // Génération de l'html avec injection des données
    let html_output = generate_html("tera", page_builder);

    // Retourner le HTML généré dans la réponse HTTP
    HttpResponse::Ok().content_type("text/html").body(html_output)
}