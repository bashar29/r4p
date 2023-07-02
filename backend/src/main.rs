use backend::configuration::get_configuration;
use backend::startup::run;
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", settings.application_port);

    let connection_pool = PgPool::connect(&settings.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let listener = std::net::TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
