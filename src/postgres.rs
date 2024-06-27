use tokio_postgres::{Error, NoTls};

pub async fn postgres() -> Result<(), Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=1234", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection Error: {}", e);
        }
    });

    let rows = client.query("select $1::text", &[&"Hello, World!"]).await?;

    let value: &str = rows[0].get(0);

    assert_eq!(value, "Hello, World!");
    Ok(())
}
