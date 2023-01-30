use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use sqlx::PgPool;
use squid::configuration::get_configuration;
use squid::errors::{ServiceError, ServiceResult};
use squid::jwt::models::Claims;
use squid::jwt::utils::decode_token;
use squid::schema::{create_schema, Context, Schema};

async fn index_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        ))
}

fn claims_from_http_request(http_req: HttpRequest) -> ServiceResult<Option<Claims>> {
    let auth_header_value = http_req.headers().get("Authorization");

    // Do I put this in the middleware instead?
    match auth_header_value {
        Some(header) => {
            let header = header
                .to_str()
                .map_err(|_| ServiceError::InternalServerError)?;
            let token: Vec<&str> = header.split("Bearer ").collect();

            if token.is_empty() {
                return Err(ServiceError::BadRequest(
                    "Missing authorization header".to_string(),
                ));
            }

            if token.len() > 2 {
                return Err(ServiceError::BadRequest(
                    "Invalid authorization header".to_string(),
                ));
            }

            let claims = decode_token(token[1])?;

            Ok(Some(claims))
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
    let claims = claims_from_http_request(http_req);
    query = query.data(claims);
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
        auth_duration_in_hours: configuration.auth_duration_in_hours,
    };
    let schema = web::Data::new(create_schema(context));

    HttpServer::new(move || {
        App::new()
            .app_data(schema.clone())
            .route("/", web::post().to(meme))
            .route("/", web::get().to(index_playground))
    })
    // TODO: Use actual config stuff
    .bind("0.0.0.0:4000")?
    .run()
    .await
}
