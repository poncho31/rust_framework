use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::Serialize;
use serde::Deserialize;
use crate::utils::builder::page_builder::list::{IntoList, ListItem};
use crate::utils::builder::page_builder::table::IntoTable;
use crate::schema::schema::{users};  // Import des schémas

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