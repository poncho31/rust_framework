use std::collections::HashMap;
use tera::{Context, Tera};
use crate::utils::template_engine::debug_template_engine::debug_template_engine;
use crate::utils::template_engine::template::html_error;



pub fn template_tera(html: HashMap<&str, String>, template_html_path: String) -> String {
    let mut context = Context::new();

    // Html sections
    for (key, value) in &html {
        context.insert(*key, value);
    }

    // Html debug
    context.insert("debug_template_engine", &debug_template_engine(context.clone().into_json()));

    // Initialiser Tera
    let tera = Tera::new("resources/views/**/*").unwrap_or_else(|e| {
        println!("Erreur lors du chargement des templates : {:?}", e);
        std::process::exit(1);
    });

    // Rendu du template
    match tera.render(&*template_html_path, &context) {
        Ok(rendered_html) => {
            println!("HTML rendu :\n{}", rendered_html);
            rendered_html
        }
        Err(e) => {
            println!("Erreur lors du rendu du template : {:?}", e);
            html_error().to_string()
        }
    }
}
