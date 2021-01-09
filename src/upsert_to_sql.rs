use crate::Result;
use std::fmt::Debug;
use tokio_postgres::{types::ToSql, GenericClient};
use tracing::instrument;

pub trait UpsertToSql {
    fn upsert_params(&self) -> Vec<&(dyn ToSql + Sync)>;

    fn upsert_query() -> &'static str;
}

#[instrument(fields(sql=T::upsert_query()), skip(client), level = "Debug", err)]
pub async fn upsert_to_sql<C, T>(client: &C, item: &T) -> Result<()>
where
    C: GenericClient,
    T: Debug + UpsertToSql,
{
    let vec = item.upsert_params();
    client.query(T::upsert_query(), &vec[..]).await?;
    Ok(())
}
