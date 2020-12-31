use crate::Result;
use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;
use tokio::spawn;
use tokio_postgres::Client;

pub async fn connect(config: &str) -> Result<Client> {
    let connector = TlsConnector::builder().build()?;
    let connector = MakeTlsConnector::new(connector);
    let (client, connection) = tokio_postgres::connect(config, connector).await?;

    spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}
