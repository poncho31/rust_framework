use serde::Serialize;
use serde_json::json;
use crate::utils::common::generate_random_string;

#[derive(Serialize, Clone, Debug)]
pub struct Form {
    pub id                 : String,
    pub title              : String,
    pub fields             : Vec<FormField>,
    pub action             : String,
    pub method             : String,
    pub submit_label       : String,
    pub template_file_path : String,
    pub ajax               : String,
}

impl Form {
    pub fn new(title: String,action: String, method: String, submit_label: String,fields: Vec<FormField>) -> Self {

        let template_file_path:String = "template/tera/form_tera.html".to_string();
        let id:String                 = format!("id_form_{}", generate_random_string(10));
        let ajax:String               = Self::ajax(id.clone(), title.clone(), fields.clone(), action.clone(), method.clone(), submit_label.clone(), template_file_path.clone());

        Self {
            id,
            title,
            fields,
            action,
            method,
            submit_label,
            template_file_path,
            ajax,
        }
    }

    // Génération d'un formulaire à partir d'une liste de champs
    pub fn create(title : String , fields: Vec<FormField>, action: String, method: String, submit_label : String) -> Self {
        Self::new(title,action, method, submit_label,fields)
    }


    pub fn ajax(
        id: String,
        title: String,
        fields: Vec<FormField>,
        action: String,
        method: String,
        submit_label: String,
        template_file_path: String,
    ) -> String {
        // Création de données spécifiques à AJAX
        let ajax_data = json!({
            "form": {
                "id": id,                         // Identifiant unique du formulaire
                "title": title,                   // Titre du formulaire
                "action": action,                 // URL où le formulaire doit être soumis
                "method": method,                 // Méthode HTTP utilisée (e.g., POST, GET)
                "submit_label": submit_label,     // Texte du bouton de soumission
                "fields": fields,                 // Liste des champs du formulaire
                "template_file_path": template_file_path // Chemin vers le fichier de template Tera
            },
            "ajax_options": {
                "validate_on_submit": true,          // Validation côté client avant d'envoyer la requête AJAX
                "send_token": true,                 // Envoi automatique d'un jeton CSRF pour sécuriser la requête
                "response_type": "json",            // Type de réponse attendu (e.g., json, html, text)
                "custom_headers": {
                    "X-Requested-With": "XMLHttpRequest", // Header pour indiquer une requête AJAX
                     "Authorization": "Bearer TOKEN"    // Exemple d'en-tête pour l'authentification avec un token
                },
                "callbacks": {
                    "before_submit": "onFormBeforeSubmit", // Fonction JS appelée avant d'envoyer la requête
                    "success": "onFormSuccess",           // Fonction JS appelée si la requête est réussie
                    "error": "onFormError",               // Fonction JS appelée en cas d'erreur
                    "complete": "onFormComplete"          // Fonction JS appelée à la fin de la requête, succès ou erreur
                },
                "request_options": {
                    "retry_on_failures": 3,               // Nombre de tentatives si une requête échoue
                    "timeout": 5000,                     // Délai d'attente maximum pour la requête (en millisecondes)
                    "cache": false,                      // Indique si les réponses doivent être mises en cache
                    "data_type": "application/json",     // Type MIME des données envoyées
                    "method_override": null              // Permet de remplacer la méthode HTTP par une autre si besoin
                },
                "ui_feedback": {
                    "loading_spinner": true,             // Activer/désactiver un spinner de chargement pendant la requête
                    "success_message": "Form submitted successfully!", // Message affiché en cas de succès
                    "error_message": "An error occurred, please try again.", // Message affiché en cas d'erreur
                    "disable_button_on_submit": true     // Désactiver le bouton de soumission pendant la requête
                },
                "debug_mode": true,                      // Activer le mode debug pour afficher des messages de développement
                "endpoint_options": {
                    "pagination": false,                 // Activer/désactiver la pagination des résultats
                    "bulk_actions": false,               // Activer/désactiver les actions groupées
                    "sort_by": ["name", "date_created"], // Colonnes disponibles pour trier les données
                    "filters": ["status", "type"],       // Liste des filtres applicables sur les résultats
                    "export_format": ["csv", "json"],    // Formats disponibles pour exporter les données
                }
            }
        });
    
        // Conversion en chaîne JSON
        ajax_data.to_string()
    }
    
}

#[derive(Serialize, Clone, Debug)]
pub enum FormFieldType {
    Text{},
    File{},
    Number{},
    Date{},
    Select {
        options  : Vec<SelectOption>,
        multiple : bool,
        debug    : bool,
    },
    TextArea{},
}

#[derive(Serialize, Clone, Debug)]
pub struct FormField {
    pub id: String,
    pub label: String,
    pub label_long: String,
    pub name: String,
    // pub disable : bool,
    // pub form    : String,
    pub field_type: FormFieldType,
    pub required: bool,
    pub placeholder: Option<String>,
}

impl FormField {
    pub fn new(
        label: &str,
        label_long: &str,
        name: &str,
        field_type: FormFieldType,
        required: bool,
        placeholder: Option<&str>,
    ) -> Self {
        Self {
            id          : format!("id_field_form_{}", generate_random_string(10)),
            label       : label.to_string(),
            label_long  : label_long.to_string(),
            name        : name.to_string(),
            field_type,
            required,
            placeholder : placeholder.map(|p| p.to_string()),
        }
    }

    pub fn new_simple(
        name: &str,
        field_type: FormFieldType,
        required: bool,
        placeholder: Option<&str>,
    ) -> Self {
        Self::new("","",name, field_type,required,placeholder)
    }

}


#[derive(Serialize, Clone, Debug)]
pub struct SelectOption {
    pub name     : String,
    pub value    : String,
    pub selected : bool,
    pub disabled : bool,
}

pub trait IntoSelectOption {
    fn to_select_option(&self) -> Vec<SelectOption>;
}

impl SelectOption {
    pub fn create<T: IntoSelectOption>(data: Vec<T>) -> Vec<SelectOption> {
        data.into_iter()
            .flat_map(|item| item.to_select_option()) // Combine les vecteurs retournés par `to_select_option`
            .collect()
    }
}
