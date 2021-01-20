use crate::{load_from_sql, FromRow, LoadFromSql, Result, UpsertToSql};
use async_trait::async_trait;
use std::fmt::Debug;
use tokio_postgres::{types::ToSql, Client, Row, ToStatement, Transaction};

#[async_trait]
pub trait Conn: Send + Sync {
    async fn execute<T>(&self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<u64>
    where
        T: ?Sized + ToStatement + Sync + Send;

    async fn query<T>(&self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>>
    where
        T: ?Sized + ToStatement + Sync + Send;

    async fn load_from_sql<T, P>(&self, params: &P) -> Result<Vec<T>>
    where
        P: Debug + Sync,
        T: FromRow + LoadFromSql<P> + Sized,
    {
        load_from_sql(self, params).await
    }

    async fn load_one_from_sql<T, P>(&self, params: &P) -> Result<Option<T>>
    where
        P: Debug + Sync,
        T: FromRow + LoadFromSql<P> + Sized,
    {
        Ok(load_from_sql(self, params).await?.pop())
    }

    async fn upsert_to_sql<T>(&self, item: &T) -> Result<()>
    where
        T: Debug + UpsertToSql + Sync,
    {
        let vec = item.upsert_params();
        self.query(T::upsert_query(), &vec[..]).await?;
        Ok(())
    }
}

#[async_trait]
impl Conn for Client {
    async fn execute<T>(&self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<u64>
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        self.execute(query, params).await.map_err(Into::into)
    }

    async fn query<T>(&self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>>
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        self.query(query, params).await.map_err(Into::into)
    }
}

#[async_trait]
impl<'a> Conn for Transaction<'a> {
    async fn execute<T>(&self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<u64>
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        self.execute(query, params).await.map_err(Into::into)
    }

    async fn query<T>(&self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>>
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        self.query(query, params).await.map_err(Into::into)
    }
}
