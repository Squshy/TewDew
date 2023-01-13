use juniper_rocket::{GraphQLRequest, GraphQLResponse};
use rocket::{routes, State};
use sqlx::PgPool;
use squid::configuration::get_configuration;
use squid::schema::{create_schema, MyContext, Schema};

#[rocket::get("/")]
fn graphiql() -> rocket::response::content::RawHtml<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

#[rocket::get("/graphql?<request>")]
async fn get_graphql_handler(
    context: &State<MyContext>,
    request: GraphQLRequest,
    schema: &State<Schema>,
) -> GraphQLResponse {
    request.execute(&schema, &context).await
}

#[rocket::post("/graphql", data = "<request>")]
async fn post_graphql_handler(
    context: &State<MyContext>,
    request: GraphQLRequest,
    schema: &State<Schema>,
) -> GraphQLResponse {
    request.execute(&schema, &context).await
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let schema = create_schema();

    let figment = rocket::Config::figment().merge(("port", configuration.application_port));
    let context = MyContext {
        db_pool: connection_pool.clone(),
    };

    let _rocket = rocket::custom(figment)
        .manage(context)
        .manage(schema)
        .mount(
            "/",
            routes![graphiql, post_graphql_handler, get_graphql_handler],
        )
        .launch()
        .await?;

    Ok(())
}
