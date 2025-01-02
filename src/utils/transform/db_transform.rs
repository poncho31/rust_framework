use serde::Serialize;


pub trait FromDbRow<T> {
    fn from_row(row: &T) -> Self;
}
pub fn get_collection_data<T, U>(data: &[T]) -> Vec<U>
where
    U: FromDbRow<T>,
{
    data.iter().map(U::from_row).collect()
}


pub trait ToViewString {
    fn to_view_string(&self) -> String;
}

impl<U> ToViewString for Vec<U>
where
    U: Serialize,
{
    fn to_view_string(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|e| format!("Error serializing to JSON: {}", e))
    }
}