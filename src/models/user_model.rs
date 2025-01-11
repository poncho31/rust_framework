use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Serialize, Deserialize};
use crate::utils::builder::page_builder::list::{IntoHtmlList, ListItem};
use crate::utils::builder::page_builder::table::IntoHtmlTable;
use crate::utils::conversion::model_conversion::{DisplayableEntity, DateFormatter};
use crate::schema::schema::users; // Import des schémas

// Structure pour la table `users`
#[derive(Queryable, Serialize, Debug, Clone)]
pub struct User {
    pub id: Option<i32>, // L'ID est nullable
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: Option<NaiveDateTime>, // Peut être nullable
}

impl DisplayableEntity for User {
    fn to_key_value_pairs(&self) -> Vec<(String, String)> {
        vec![
            ("ID".to_string(), self.id.map_or_else(|| "-".to_string(), |v| v.to_string())),
            ("Nom d'utilisateur".to_string(), self.username.clone()),
            ("Email".to_string(), self.email.clone()),
            (
                "Créé le".to_string(),
                self.created_at
                    .as_ref()
                    .map_or_else(|| "Non défini".to_string(), |date| NaiveDateTime::format_date(date)),
            ),
        ]
    }
}

impl IntoHtmlTable for User {
    fn headers() -> Vec<String> {
        vec![
            "ID".to_string(),
            "Nom d'utilisateur".to_string(),
            "Email".to_string(),
            "Créé le".to_string(),
        ]
    }

    fn to_row(&self) -> Vec<String> {
        self.to_key_value_pairs().into_iter().map(|(_, v)| v).collect()
    }
}

impl IntoHtmlList for User {
    fn to_list_item(&self) -> ListItem {
        ListItem {
            data: self.to_key_value_pairs(),
        }
    }
}

// Structure pour l'insertion d'un nouvel utilisateur
#[derive(Insertable)]
#[diesel(table_name = users)] // Spécification de la table cible
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
}

// Structure pour les données provenant du formulaire
#[derive(Deserialize, Serialize)]
pub struct NewUserData {
    pub username: String,
    pub email: String,
    pub password: String, // Le mot de passe brut provenant du formulaire
    pub created_at: Option<String>, // Date de création au format chaîne (peut être nullable)
}

impl NewUserData {
    pub fn to_new_user(&self) -> NewUser {
        NewUser {
            username: &self.username,
            email: &self.email,
            password_hash: &self.password, // Utiliser un hash sécurisé dans un vrai cas
        }
    }

    pub fn parse_created_at(&self) -> Option<NaiveDateTime> {
        self.created_at
            .as_deref()
            .map(|date| NaiveDateTime::parse_date(date))
    }
}
