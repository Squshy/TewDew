use async_graphql::http::GraphiQLSource;
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use rocket::{response::content, routes, State};
use sqlx::PgPool;
use squid::configuration::get_configuration;
use squid::schema::{create_schema, Context, Schema};

#[rocket::get("/")]
fn graphiql() -> content::RawHtml<String> {
    content::RawHtml(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[rocket::get("/graphql?<query>")]
async fn graphql_query(
    // context: &State<Context>,
    query: GraphQLQuery,
    schema: &State<Schema>,
) -> GraphQLResponse {
    query.execute(schema.inner()).await
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
async fn graphql_request(
    // context: &State<Context>,
    request: GraphQLRequest,
    schema: &State<Schema>,
) -> GraphQLResponse {
    request.execute(schema.inner()).await
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let context = Context {
        db_pool: connection_pool.clone(),
    };
    let schema = create_schema(context.clone());

    let figment = rocket::Config::figment().merge(("port", configuration.application_port));

    let _rocket = rocket::custom(figment)
        .manage(context)
        .manage(schema)
        .mount("/", routes![graphiql, graphql_query, graphql_request])
        .launch()
        .await?;

    Ok(())
}
