use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::Queryable;


use crate::schema::schema::{users, events};  // Import des schémas
use serde::Serialize;
use serde::Deserialize;
use crate::utils::builder::page_builder::list::{IntoList, ListItem};
use crate::utils::builder::page_builder::table::IntoTable;

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



// Structure pour la table `users`
#[derive(Queryable, Serialize, Debug)]
pub struct User {
    pub id: Option<i32>,  // L'ID est nullable
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: Option<NaiveDateTime>,  // Peut être nullable
}


// Structure pour l'insertion d'un nouvel utilisateur
#[derive(Insertable)]
#[diesel(table_name = users)]  // Spécification de la table cible
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
}

#[derive(Deserialize, Serialize)]
pub struct NewUserData {
    pub username: String,
    pub email: String,
    pub password: String,  // Format de date en chaîne de caractères (car cela vient d'un formulaire)
    pub created_at: Option<String>,  // Format de date en chaîne de caractères (car cela vient d'un formulaire)
}

impl NewUserData {
    pub fn to_new_user(&self) -> NewUser {
        NewUser {
            username: &self.username,
            email: &self.email,
            password_hash: &self.password,
        }
    }
}