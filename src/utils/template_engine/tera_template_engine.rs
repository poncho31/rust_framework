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

    // Ajouter la fonction IncludeHtml avec une copie du contexte principal
    tera.register_function("IncludeHtml", IncludeHtml {
        tera: tera.clone(),
        main_context: context.clone(), // Passer une copie du contexte
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
    main_context: Context, // Utiliser une copie du contexte
}

impl Function for IncludeHtml {
    fn call(&self, args: &HashMap<String, Value>) -> TeraResult<Value> {
        // Vérifier que le chemin est fourni
        if let Some(Value::String(file_path)) = args.get("file_path") {
            let file_path = file_path.trim(); // Normaliser le chemin
            println!("DEBUG - Chemin fourni à IncludeHtml : {}", file_path);

            // Vérifier si le template existe avant de le rendre
            if !self.tera.get_template_names().any(|name| name == file_path) {
                return Err(format!("Template '{}' introuvable dans Tera.", file_path).into());
            }

            // Rendre le fichier avec une copie du contexte
            let rendered = self.tera.render(file_path, &self.main_context)?;
            Ok(Value::String(rendered))
        } else {
            Err("Argument 'file_path' manquant ou invalide.".into())
        }
    }
}

