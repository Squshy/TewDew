use juniper_rocket::{GraphQLRequest, GraphQLResponse};
use rocket::{routes, State};
use squid::configuration::get_configuration;
use squid::schema::{create_schema, Schema};

#[rocket::get("/")]
fn graphiql() -> rocket::response::content::RawHtml<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

#[rocket::get("/graphql?<request>")]
async fn get_graphql_handler(schema: &State<Schema>, request: GraphQLRequest) -> GraphQLResponse {
    request.execute(&schema, &()).await
}

#[rocket::post("/graphql", data = "<request>")]
async fn post_graphql_handler(schema: &State<Schema>, request: GraphQLRequest) -> GraphQLResponse {
    request.execute(&schema, &()).await
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let schema = create_schema();

    let figment = rocket::Config::figment().merge(("port", configuration.application_port));

    let _rocket = rocket::custom(figment)
        .manage(schema)
        .mount(
            "/",
            routes![graphiql, post_graphql_handler, get_graphql_handler],
        )
        .launch()
        .await?;

    Ok(())
}
