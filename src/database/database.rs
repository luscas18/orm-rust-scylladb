use scylla::{Session, Error, IntoTypedRows};
use std::env;

pub async fn establish_connection() -> Result<Session, Error> {
    let scylla_uri = env::var("SCYLLA_URI").unwrap_or("127.0.0.1:9042".to_string());
    let session = Session::connect(scylla_uri, None).await?;
    Ok(session)
}