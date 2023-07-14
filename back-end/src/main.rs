use dotenv::dotenv;
use std::{env::var, net::SocketAddr};

mod database;
mod router;
mod utils;

use {database::db::establish_connection, router::router::mount_router};

mod generics {
    use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};
    pub type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let connection_pool = establish_connection().await;

    let router = mount_router(connection_pool);
    let port = var("SERVER_PORT").unwrap().parse::<u16>().unwrap();
    let server_addr = SocketAddr::from(([0, 0, 0, 0], port));

    tracing::info!("listening on {}", server_addr);

    axum::Server::bind(&server_addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
