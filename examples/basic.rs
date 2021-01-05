use pg_orm::{connect, load_from_sql, upsert_to_sql, LoadFromSql, UpsertToSql};
use std::error::Error;

#[derive(Debug, LoadFromSql, UpsertToSql)]
#[table("TempUsers")]
pub struct User {
    #[key]
    pub id: i32,
    pub name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let conn_str = "host=localhost user=postgres password=cot-ci-pw";
    let client = connect(conn_str).await?;

    let sql = r"CREATE TEMP TABLE TempUsers (
        id int NOT NULL PRIMARY KEY,
        name TEXT NOT NULL
    )";

    client.execute(sql, &[]).await?;

    let user = User {
        id: 3,
        name: "John Doe".to_string(),
    };

    upsert_to_sql(&client, &user).await?;

    let users: Vec<User> = load_from_sql(&client, &(None,)).await?;

    println!("{:?}", users);

    Ok(())
}
