use std::net::TcpListener;
use training_crab::configuration::get_configurations;
use training_crab::startup::run;
use training_crab::telemetry::{get_subscriber, init_subscriber};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("training_crab".into(), "info".into());
    init_subscriber(subscriber);

    let configuration = get_configurations().expect("Failed to read configuration.");
    let database = configuration.database.db().await.unwrap();

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    let firebase_secret_path = configuration.firebase.secret_path;
    run(listener, database, firebase_secret_path)?.await?;
    Ok(())
}
