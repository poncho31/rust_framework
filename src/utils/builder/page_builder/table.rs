use serde::Serialize;
use crate::models::models::Event;

#[derive(Serialize, Clone, Debug)]
pub struct Table {
    pub id: String,        // Identifiant unique pour la table
    pub headers: Vec<String>, // Liste des en-têtes
    pub rows: Vec<Vec<String>>, // Liste des lignes, chaque ligne contenant des cellules
    pub file_path : String
}

impl Table {
    /// Crée une nouvelle table
    pub fn new(id: &str, headers: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        Self {
            id: id.to_string(),
            headers,
            rows,
            file_path: "template/tera/table_tera.html".to_string(),
        }
    }

    /// Convertit les données des événements en une table
    pub fn from_events(id: &str, events: Vec<Event>) -> Self {
        let headers = vec![
            "ID".to_string(),
            "Nom de l'événement".to_string(),
            "Date".to_string(),
        ];

        let rows: Vec<Vec<String>> = events
            .into_iter()
            .map(|event| {
                vec![
                    event.id.map(|v| v.to_string()).unwrap_or_else(|| "-".to_string()), // Valeur par défaut
                    event.title.clone(),
                    event.date.format("%Y-%m-%d").to_string(),
                ]
            })
            .collect();

        Self::new(id, headers, rows)
    }
}
