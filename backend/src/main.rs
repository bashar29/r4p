use backend::configuration::get_configuration;
use backend::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", settings.application_port);

    let listener = std::net::TcpListener::bind(address)?;
    run(listener)?.await
}
