use crate::{Conn, FromRow, Result};
use std::fmt::Debug;
use tokio_postgres::types::ToSql;
use tracing::instrument;

pub trait LoadFromSql<P> {
    fn load_from_sql_query() -> String;

    fn load_from_sql_params(params: &P) -> Vec<&(dyn ToSql + Sync)>;
}

#[instrument(level = "Debug", err, skip(client), fields(sql=%T::load_from_sql_query()))]
pub async fn load_from_sql<C: ?Sized, T, P>(client: &C, params: &P) -> Result<Vec<T>>
where
    C: Conn,
    P: Debug,
    T: FromRow + LoadFromSql<P> + Sized,
{
    let sql = T::load_from_sql_query();
    let params = T::load_from_sql_params(params);

    Ok(client
        .query(sql.as_str(), &params[..])
        .await?
        .into_iter()
        .map(FromRow::from_row)
        .collect())
}
