use tokio_postgres::{Error, NoTls};

async fn postgres() -> Result<(), Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=1234", NoTls).await?;

    if let Err(e) = connection.await {
        eprintln!("Connection Error: {}", e);
    }
    Ok(())
}
