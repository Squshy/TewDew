use sqlx::{Connection, PgConnection};
use squid::configuration::get_configuration;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_string = configuration.database.connection_string();

    let _connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres");

    let figment = rocket::Config::figment().merge(("port", configuration.application_port));

    let _rocket = rocket::custom(figment)
        .mount("/", routes![index])
        .launch()
        .await?;

    Ok(())
}
