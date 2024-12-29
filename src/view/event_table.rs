use crate::models::models::Event;
use crate::utils::builder::page::module::section::TableTrait;
use crate::utils::transform::db_transform::FromDbRow;

#[derive(Debug)]
pub struct EventItem {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub date: String, // Ajustez le type si nécessaire
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

pub struct EventTable {
    pub items: Vec<EventItem>,
}

impl EventTable {
    pub fn new(items: Vec<EventItem>) -> Self {
        EventTable { items }
    }
}


impl TableTrait for EventTable {
    fn to_html(&self) -> String {
        let rows_html: Vec<String> = self.items.iter().map(|item| {
            format!(
                r#"<li class="box">
                    <h3 class="title is-4">{}</h3>
                    <p><strong>Date :</strong> {}</p>
                    <p><strong>Description :</strong> {}</p>
                </li>"#,
                item.title,
                item.date,
                item.description.clone().unwrap_or_else(|| "Aucune description".to_string())
            )
        }).collect();

        format!(r#"<ul id="event_list">{}</ul>"#, rows_html.join("\n"))
    }
}
