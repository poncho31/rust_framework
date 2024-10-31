// TABLE
pub struct Table {
    id: String,
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl Table {
    pub fn new(id: String, headers: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        Table { id, headers, rows }
    }

    pub fn to_html(&self) -> String {
        // Génération des entêtes de colonnes
        let headers_html = self.headers.iter()
            .map(|header| format!("<th>{}</th>", header))
            .collect::<Vec<_>>()
            .join("\n");

        // Génération des lignes de données
        let rows_html = self.rows.iter().map(|row| {
            let cells = row.iter()
                .map(|cell| format!("<td>{}</td>", cell))
                .collect::<Vec<_>>()
                .join("\n");
            format!("<tr>{}</tr>", cells)
        }).collect::<Vec<_>>()
            .join("\n");

        // Code HTML complet de la table
        format!(
            r#"
            <table id="{id}">
                <thead>
                    <tr>{headers}</tr>
                </thead>
                <tbody>
                    {rows}
                </tbody>
            </table>
            "#,
            id = self.id,
            headers = headers_html,
            rows = rows_html
        )
    }
}


// EVENT TABLE

use chrono::NaiveDateTime;
// EventItem pour représenter chaque événement
pub struct EventItem {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub date: NaiveDateTime,
}

// EventTable pour afficher la liste des événements
pub struct EventTable {
    items: Vec<EventItem>,
}

impl EventTable {
    pub fn new(items: Vec<EventItem>) -> Self {
        EventTable { items }
    }

    pub fn to_html(&self) -> String {
        let rows_html: Vec<String> = self.items.iter().map(|item| {
            format!(
                r#"<li class="event-item">
                    <h3>{}</h3>
                    <p>{}</p>
                    <span>{}</span>
                </li>"#,
                item.title,
                item.description.clone().unwrap_or_default(),
                item.date.format("%Y-%m-%d %H:%M")
            )
        }).collect();

        format!(r#"<ul id="event_list">{}</ul>"#, rows_html.join("\n"))
    }
}
