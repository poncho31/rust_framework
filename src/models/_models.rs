use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::Queryable;


use crate::schema::_schema::{users, events};  // Import des schémas
use serde::Serialize;

// Structure pour la table `users`
#[derive(Queryable, Serialize, Debug)]
pub struct User {
    pub id: Option<i32>,  // L'ID est nullable
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: Option<NaiveDateTime>,  // Peut être nullable
}

// Structure pour la table `events`
#[derive(Queryable, Serialize, Debug)]
pub struct Event {
    pub id: Option<i32>,  // L'ID est nullable
    pub title: String,
    pub description: Option<String>,
    pub date: NaiveDateTime,  // Mapping vers Timestamp
    pub user_id: i32,
}

// Structure pour l'insertion d'un nouvel utilisateur
#[derive(Insertable)]
#[diesel(table_name = users)]  // Spécification de la table cible
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
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
