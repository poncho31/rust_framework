use std::fmt;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use crate::schema::schema::{events};  // Import des schémas
use serde::Serialize;
use serde::Deserialize;
use crate::utils::builder::page_builder::list::{IntoList, ListItem};
use crate::utils::builder::page_builder::table::IntoTable;
use crate::utils::transform::db_transform::{FromDbRow, ToViewString};

// Structure pour la table `events`
#[derive(Queryable, Serialize, Debug, Clone)]
pub struct Event {
    pub id: Option<i32>,  // L'ID est nullable
    pub title: String,
    pub description: Option<String>,
    pub date: NaiveDateTime,  // Mapping vers Timestamp
    pub user_id: i32,
}

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
            self.id.map(|v| v.to_string()).unwrap_or_else(|| "-".to_string()),
            self.title.clone(),
            self.description.clone().unwrap_or_else(|| "-".to_string()),
            self.date.format("%Y-%m-%d").to_string(),
        ]
    }
}

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




#[derive(Debug, Serialize)]
pub struct EventItem {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub date: String, // Ajustez le type si nécessaire
}

impl fmt::Display for EventItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => write!(f, "{}", json),
            Err(e) => write!(f, "Error serializing to JSON: {}", e),
        }
    }
}
impl ToViewString for EventItem {
    fn to_view_string(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|e| format!("Error serializing to JSON: {}", e))
    }
}


impl FromDbRow<Event> for EventItem {
    fn from_row(event: &Event) -> Self {
        Self {
            id: event.id.expect("ID manquant"),
            title: event.title.clone(),
            description: event.description.clone(),
            date: event.date.to_string(),
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
    pub date: String,  // Format de date en chaîne de caractères (car cela vient d'un formulaire)
    pub user_id: i32,
}

impl NewEventData {
    pub fn to_new_event(&self) -> NewEvent {
        NewEvent {
            title: &self.title,
            description: self.description.as_deref(),
            date: NaiveDateTime::parse_from_str(&self.date, "%Y-%m-%d %H:%M:%S").unwrap(),
            user_id: self.user_id,
        }
    }
}

