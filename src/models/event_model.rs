use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use crate::schema::schema::events;
use crate::utils::builder::page_builder::widget::IntoHtmlWidget; // Import des schémas
use serde::{Serialize, Deserialize};
use crate::utils::builder::page_builder::form::{IntoSelectOption, SelectOption};
use crate::utils::builder::page_builder::list::{IntoHtmlList, ListItem};
use crate::utils::builder::page_builder::table::IntoHtmlTable;
use crate::utils::conversion::model_conversion::{DisplayableEntity, DateFormatter};

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct Event {
    pub id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub date: NaiveDateTime,
    pub user_id: i32,
}

impl DisplayableEntity for Event {
    fn to_key_value_pairs(&self) -> Vec<(String, String)> {
        vec![
            ("ID".to_string(), self.id.map_or_else(|| "-".to_string(), |v| v.to_string())),
            ("Titre".to_string(), self.title.clone()),
            (
                "Description".to_string(),
                self.description.clone().unwrap_or_else(|| "Aucune description".to_string()),
            ),
            ("Date".to_string(), NaiveDateTime::format_date(&self.date)),
        ]
    }
}

impl IntoHtmlTable for Event {
    fn headers() -> Vec<String> {
        vec![
            "ID".to_string(),
            "Nom de l'événement".to_string(),
            "Description".to_string(),
            "Date".to_string(),
        ]
    }

    fn to_row(&self) -> Vec<String> {
        self.to_key_value_pairs().into_iter().map(|(_, v)| v).collect()
    }
}

impl IntoHtmlWidget for Event {
    fn headers() -> Vec<String> {
        vec![
            "ID".to_string(),
            "Nom de l'événement".to_string(),
            "Description".to_string(),
            "Date".to_string(),
        ]
    }

    fn to_row(&self) -> Vec<String> {
        self.to_key_value_pairs().into_iter().map(|(_, v)| v).collect()
    }
}

impl IntoHtmlList for Event {
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
    pub date: String,
    pub user_id: i32,
}

impl NewEventData {
    // Convertit `NewEventData` en `NewEvent`
    pub fn new(&self) -> NewEvent {
        NewEvent {
            title: &self.title,
            description: self.description.as_deref(),
            date: NaiveDateTime::parse_date(&self.date),
            user_id: self.user_id,
        }
    }
}



// OPTIONS pour
impl IntoSelectOption for Vec<Event> {
    fn to_select_option(&self) -> Vec<SelectOption> {
        self.iter()
            .map(|event| SelectOption {
                name: event.id.map_or("-".to_string(), |v| v.to_string()),
                value: event.title.clone(), 
                selected: false,
                disabled: false,
            })
            .collect()
    }
}


