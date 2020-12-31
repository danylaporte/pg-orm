use tokio_postgres::Row;

pub trait FromRow {
    fn from_row(row: Row) -> Self;
}
