use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use std::env::var;

use crate::generics::Pool;

pub async fn establish_connection() -> Pool {
    let db_url = var("DATABASE_URL").unwrap();

    // set up connection pool
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
    bb8::Pool::builder().build(config).await.unwrap()
}
