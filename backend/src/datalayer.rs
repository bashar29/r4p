use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::domain::NewAccount;

/*
let mut transaction = pool
.begin()
.await
.context("Failed to acquire a Postgres connection from the pool")?;
    //transaction: &mut Transaction<'_, Postgres>,

*/

pub async fn create_account(pool: PgPool, new_account: NewAccount) -> Result<Uuid, sqlx::Error> {
    let uuid = Uuid::new_v4();

    Ok(uuid)
}
