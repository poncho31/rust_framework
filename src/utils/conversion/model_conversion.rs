use chrono::NaiveDateTime;
use serde::Serialize;

// Trait commun pour les entités affichables
pub trait DisplayableEntity: Serialize {
    // Génère une liste de paires clé-valeur pour les affichages
    fn to_key_value_pairs(&self) -> Vec<(String, String)>;
}

// Trait pour la gestion des formats de dates
pub trait DateFormatter {
    fn parse_date(date: &str) -> NaiveDateTime;
    fn format_date(date: &NaiveDateTime) -> String;
}

// Implémentation pour les dates
impl DateFormatter for NaiveDateTime {
    fn parse_date(date: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S")
            .expect("Format de date invalide")
    }

    fn format_date(date: &NaiveDateTime) -> String {
        date.format("%Y-%m-%d").to_string()
    }
}
