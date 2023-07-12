use backend::configuration;
use sqlx::PgPool;
use std::net::TcpListener;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let settings = configuration::get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&settings.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let server =
        backend::startup::run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db_pool: connection_pool,
    }
}
