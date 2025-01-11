use std::collections::HashMap;
use serde_json::Value;
use tera::{Context, Tera, Function, Result as TeraResult};
use crate::utils::template_engine::template::html_error;



pub fn template_tera(html: HashMap<&str, Value>, template_html_path: String) -> String {
    let mut context = Context::new();

    // Parcourir les sections HTML et ajouter au contexte
    for (key, value) in &html {
        println!("KEYYYY {} VALUEE : {}", key, value);
        context.insert(*key, value);
    }

    // Initialiser Tera
    let mut tera = Tera::new("resources/views/**/*").unwrap_or_else(|e| {
        println!("Erreur lors du chargement des templates : {:?}", e);
        std::process::exit(1);
    });

    // Enregistrer la fonction debug
    tera.register_function("debug", Debug);


    // Rendu du template
    match tera.render(&*template_html_path, &context) {
        Ok(rendered_html) => {
            println!("HTML rendu :\n{}", rendered_html);
            rendered_html
        }
        Err(e) => {
            println!("Erreur lors du rendu du template : {:?}", e);
            html_error(e.to_string()).to_string()
        }
    }
}

use serde_json::to_string_pretty;
pub struct Debug;

impl Function for Debug {
    fn call(&self, args: &HashMap<String, Value>) -> TeraResult<Value> {
        if let Some(value) = args.get("value") {
            // Convertir la valeur en JSON formaté
            match to_string_pretty(value) {
                Ok(json) => Ok(Value::String(json)),
                Err(_) => Ok(Value::String("Erreur lors de la conversion en JSON".to_string())),
            }
        } else {
            Ok(Value::String("Aucune donnée fournie à debug".to_string()))
        }
    }
}
