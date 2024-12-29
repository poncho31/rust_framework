pub trait FromDbRow<T> {
    fn from_row(row: &T) -> Self;
}
pub fn get_collection_data<T, U>(data: &[T]) -> Vec<U>
where
    U: FromDbRow<T>,
{
    data.iter().map(U::from_row).collect()
}
