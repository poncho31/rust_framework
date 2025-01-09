use std::fmt;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use crate::schema::schema::events; // Import des schémas
use serde::{Serialize, Deserialize};
use crate::utils::builder::page_builder::list::{IntoList, ListItem};
use crate::utils::builder::page_builder::table::IntoTable;

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

    /// Utilitaire pour les valeurs optionnelles avec valeur par défaut
    fn unwrap_or_default<T: ToString>(value: Option<T>, default: &str) -> String {
        value.map_or_else(|| default.to_string(), |v| v.to_string())
    }

    /// Convertit l'événement en une liste de paires clé-valeur (utile pour table ou liste)
    fn to_key_value_pairs(&self) -> Vec<(String, String)> {
        vec![
            ("ID".to_string(), Self::unwrap_or_default(self.id, "-")),
            ("Titre".to_string(), self.title.clone()),
            (
                "Description".to_string(),
                Self::unwrap_or_default(self.description.clone(), "Aucune description"),
            ),
            ("Date".to_string(), self.date.format("%Y-%m-%d").to_string()),
        ]
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
        vec!["ID".to_string(), "Nom de l'événement".to_string(), "Description".to_string(), "Date".to_string()]
    }

    fn to_row(&self) -> Vec<String> {
        self.to_key_value_pairs().iter().map(|(_, v)| v.clone()).collect()
    }
}

// Implémentation pour l'affichage en liste HTML
impl IntoList for Event {
    fn to_list_item(&self) -> ListItem {
        ListItem {
            data: self.to_key_value_pairs(),
        }
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
