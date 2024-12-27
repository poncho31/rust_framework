use crate::utils::builder::page::module::section::TableTrait;

pub struct EventItem {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub date: String, // Ajustez le type si nécessaire
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
