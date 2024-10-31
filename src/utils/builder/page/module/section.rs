use crate::utils::builder::page::module::table::Table;

pub trait TableTrait {
    fn to_html(&self) -> String;
}

pub struct Section<T> {
    id: String,
    title: String,
    description: Option<String>,
    table: Option<T>,
}

impl<T> Section<T>
where
    T: TableTrait, // Contrainte pour garantir que T implémente TableTrait
{
    pub fn new(title: String, description: Option<String>, table: Option<T>) -> Self {
        Section {
            id: format!("section_{}", &title),
            title,
            description,
            table,
        }
    }

    pub fn to_html(&self) -> String {
        let description_html = self.description.clone().unwrap_or_default();
        let table_html = if let Some(ref table) = self.table {
            table.to_html()
        } else {
            String::new()
        };

        format!(
            r#"
            <section id="{id}">
                <h2>{title}</h2>
                <p>{description}</p>
                {table}
            </section>
            "#,
            id = self.id,
            title = self.title,
            description = description_html,
            table = table_html
        )
    }
}
