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

    // Ajouter la fonction IncludeHtml avec des instances possédées
    tera.register_function("IncludeHtml", IncludeHtml {
        tera: tera.clone(),         // Cloner l'instance de Tera
        main_context: context.clone(), // Cloner le contexte
    });

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


pub struct IncludeHtml {
    tera: Tera,
    main_context: Context,
}

impl Function for IncludeHtml {
    fn call(&self, args: &HashMap<String, Value>) -> TeraResult<Value> {
        if let Some(Value::String(file_path)) = args.get("file_path") {
            let file_path = file_path.trim();

            if !self.tera.get_template_names().any(|name| name == file_path) {
                return Err(format!("Template '{}' introuvable dans Tera.", file_path).into());
            }

            match self.tera.render(file_path, &self.main_context) {
                Ok(rendered_html) => Ok(Value::String(rendered_html)),
                Err(e) => Err(format!("Erreur lors du rendu du template '{}': {}", file_path, e).into()),
            }
        } else {
            Err("Argument 'file_path' manquant ou invalide.".into())
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
