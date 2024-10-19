use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::schema::{users, events};

// Structure pour la table `users`
#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: Option<NaiveDateTime>, // Option car SQLite gère la valeur par défaut
}

// Structure pour la table `events`
#[derive(Queryable)]
pub struct Event {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub date: NaiveDateTime,
    pub user_id: i32,
}

// Structure pour l'insertion d'un nouvel utilisateur
#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
}
