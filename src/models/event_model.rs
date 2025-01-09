use std::fmt;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use crate::schema::schema::events; // Import des schémas
use serde::{Serialize, Deserialize};
use crate::utils::builder::page_builder::list::{IntoList, ListItem};
use crate::utils::builder::page_builder::table::IntoTable;
use crate::utils::transform::db_transform::{FromDbRow, ToViewString};

// Structure principale pour la table `events`
#[derive(Queryable, Serialize, Debug, Clone)]
pub struct Event {
    pub id: Option<i32>,  // L'ID est nullable
    pub title: String,
    pub description: Option<String>,
    pub date: NaiveDateTime,
    pub user_id: i32,
}

impl Event {
    /// Convertit l'événement en un format JSON
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|e| format!("Erreur : {}", e))
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_json())
    }
}

// Implémentation pour l'affichage en table HTML
impl IntoTable for Event {
    fn headers() -> Vec<String> {
        vec![
            "ID".to_string(),
            "Nom de l'événement".to_string(),
            "Description".to_string(),
            "Date".to_string(),
        ]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            self.id.map_or_else(|| "-".to_string(), |id| id.to_string()),
            self.title.clone(),
            self.description.clone().unwrap_or_else(|| "-".to_string()),
            self.date.format("%Y-%m-%d").to_string(),
        ]
    }
}

// Implémentation pour l'affichage en liste HTML
impl IntoList for Event {
    fn to_list_item(&self) -> ListItem {
        ListItem {
            data: vec![
                ("Titre".to_string(), self.title.clone()),
                ("Date".to_string(), self.date.format("%Y-%m-%d").to_string()),
                (
                    "Description".to_string(),
                    self.description.clone().unwrap_or_else(|| "Aucune description".to_string()),
                ),
            ],
        }
    }
}

// Implémentation de `FromDbRow`
impl FromDbRow<Event> for Event {
    fn from_row(event: &Event) -> Self {
        Self {
            id: event.id,
            title: event.title.clone(),
            description: event.description.clone(),
            date: event.date,
            user_id: event.user_id,
        }
    }
}
// Implémentation de `ToViewString` pour `Event`
impl ToViewString for Event {
    fn to_view_string(&self) -> String {
        self.to_json() // Appelle simplement la méthode `to_json` existante
    }
}


// Structure pour l'insertion d'un nouvel événement
#[derive(Insertable)]
#[diesel(table_name = events)]
pub struct NewEvent<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub date: NaiveDateTime,
    pub user_id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct NewEventData {
    pub title: String,
    pub description: Option<String>,
    pub date: String, // Format de date en chaîne (issu d'un formulaire)
    pub user_id: i32,
}

impl NewEventData {
    /// Convertit `NewEventData` en `NewEvent`
    pub fn to_new_event(&self) -> NewEvent {
        NewEvent {
            title: &self.title,
            description: self.description.as_deref(),
            date: NaiveDateTime::parse_from_str(&self.date, "%Y-%m-%d %H:%M:%S")
                .expect("Format de date invalide"),
            user_id: self.user_id,
        }
    }
}
