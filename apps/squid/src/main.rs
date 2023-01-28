use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use sqlx::PgPool;
use squid::configuration::get_configuration;
use squid::errors::{ServiceError, ServiceResult};
use squid::schema::{create_schema, Context, Schema};

async fn index_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        ))
}

fn hehe(http_req: HttpRequest) -> ServiceResult<Option<String>> {
    let tee_hee = http_req.headers().get("Authorization");

    match tee_hee {
        Some(val) => {
            let h = val
                .to_str()
                .map_err(|_| ServiceError::InternalServerError)?;
            Ok(Some(h.to_string()))
        }
        None => Ok(None),
    }
}

async fn meme(
    schema: web::Data<Schema>,
    http_req: HttpRequest,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut query = req.into_inner();
    let meme = hehe(http_req);
    query = query.data(meme);
    schema.execute(query).await.into()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let context = Context {
        db_pool: connection_pool.clone(),
    };
    let schema = web::Data::new(create_schema(context));

    HttpServer::new(move || {
        App::new()
            .app_data(schema.clone())
            .route("/", web::post().to(meme))
            .route("/", web::get().to(index_playground))
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await
}
