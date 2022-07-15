//! Database interface

use sqlx::AnyPool;

use crate::Error;

pub async fn open(url: &str) -> Result<AnyPool, Error> {
    let pool = AnyPool::connect(url)
        .await
        .map_err(Error::DatabaseOpenError)?;

    Ok(pool)
}
