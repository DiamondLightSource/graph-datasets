#![forbid(unsafe_code)]
#![doc=include_str!("../../README.md")]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

/// GraphQL resolvers
mod graphql;

use async_graphql::{extensions::Tracing, http::GraphiQLSource, SDLExportOptions};
use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::{response::Html, routing::get, Router};
use clap::Parser;
use graphql::{root_schema_builder, RootSchema};
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr, TransactionError};
use std::{
    fs::File,
    io::Write,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    path::PathBuf,
};
use tokio::net::TcpListener;
use tracing::instrument;
use url::Url;

/// A service providing Beamline ISPyB data collected during sessions
#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
enum Cli {
    /// Starts a webserver serving the GraphQL API
    Serve(ServeArgs),
    /// Produces the GraphQL schema
    Schema(SchemaArgs),
}

/// Arguments for serving the GraphQL API
#[derive(Debug, Parser)]
struct ServeArgs {
    /// The port to which this application should bind
    #[arg(short, long, env = "PORT", default_value_t = 80)]
    port: u16,
    /// The URL of the ISPyB instance which should be connected to
    #[arg(long, env = "DATABASE_URL")]
    database_url: Url,
    /// The [`tracing::Level`] to log at
    #[arg(long, env = "LOG_LEVEL", default_value_t = tracing::Level::INFO)]
    log_level: tracing::Level,
    /// The URL of the OpenTelemetry collector to send traces to
    #[arg(long, env = "OTEL_COLLECTOR_URL")]
    otel_collector_url: Option<Url>,
}

/// Arguments for produces the GraphQL schema
#[derive(Debug, Parser)]
struct SchemaArgs {
    /// The path to write the schema to, if not set the schema will be printed to stdout
    #[arg(short, long)]
    path: Option<PathBuf>,
}

/// Creates a connection pool to access the database
#[instrument(skip(database_url))]
async fn setup_database(database_url: Url) -> Result<DatabaseConnection, TransactionError<DbErr>> {
    let connection_options = ConnectOptions::new(database_url.to_string());
    let connection = Database::connect(connection_options).await?;
    Ok(connection)
}

/// Creates an [`axum::Router`] serving GraphiQL, synchronous GraphQL and GraphQL subscriptions
fn setup_router(schema: RootSchema) -> Router {
    #[allow(clippy::missing_docs_in_private_items)]
    const GRAPHQL_ENDPOINT: &str = "/";
    #[allow(clippy::missing_docs_in_private_items)]
    const SUBSCRIPTION_ENDPOINT: &str = "/ws";

    Router::new()
        .route(
            GRAPHQL_ENDPOINT,
            get(Html(
                GraphiQLSource::build()
                    .endpoint(GRAPHQL_ENDPOINT)
                    .subscription_endpoint(SUBSCRIPTION_ENDPOINT)
                    .finish(),
            ))
            .post_service(GraphQL::new(schema.clone())),
        )
        .route_service(SUBSCRIPTION_ENDPOINT, GraphQLSubscription::new(schema))
}

/// Serves the endpoints on the specified port forever
async fn serve(router: Router, port: u16) -> Result<(), std::io::Error> {
    let socket_addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port));
    let listener = TcpListener::bind(socket_addr).await?;
    println!("GraphiQL IDE: {}", socket_addr);
    axum::serve(listener, router.into_make_service()).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let args = Cli::parse();

    match args {
        Cli::Serve(args) => {
            let database = setup_database(args.database_url).await.unwrap();
            let schema = root_schema_builder()
                .extension(Tracing)
                .data(database)
                .finish();
            let router = setup_router(schema);
            serve(router, args.port).await.unwrap();
        }
        Cli::Schema(args) => {
            let schema = root_schema_builder().finish();
            let schema_string = schema.sdl_with_options(SDLExportOptions::new().federation());
            if let Some(path) = args.path {
                let mut file = File::create(path).unwrap();
                file.write_all(schema_string.as_bytes()).unwrap();
            } else {
                println!("{}", schema_string)
            }
        }
    }
}
