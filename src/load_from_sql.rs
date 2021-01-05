use crate::{FromRow, Result};
use std::fmt::Debug;
use tokio_postgres::{types::ToSql, Client};
use tracing::instrument;

pub trait LoadFromSql<P> {
    fn load_from_sql_query() -> String;

    fn load_from_sql_params(params: &P) -> Vec<&(dyn ToSql + Sync)>;
}

#[instrument(level = "Debug", err, fields(sql=%T::load_from_sql_query()))]
pub async fn load_from_sql<T, P>(client: &Client, params: &P) -> Result<Vec<T>>
where
    P: Debug,
    T: FromRow + LoadFromSql<P> + Sized,
{
    let sql = T::load_from_sql_query();
    let params = T::load_from_sql_params(params);
    let statement = client.prepare(&sql).await?;

    Ok(client
        .query(&statement, &params[..])
        .await?
        .into_iter()
        .map(FromRow::from_row)
        .collect())
}
