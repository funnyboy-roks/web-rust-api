use std::env;

use axum::{response::Redirect, routing::get, Router};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod blog;
mod models;
mod paper;
mod site;

use models::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let projects = projects::Project::load().expect("Error when loading projects.");
    println!("Loaded {} projects.", projects.len());

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_todos=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our application with a single route
    let app = Router::new()
        .nest("/paper", paper::router())
        .nest("/blog", blog::router())
        .nest("/site", site::router(projects))
        .route(
            "/discord",
            get(|| async {
                Redirect::permanent(&env::var("DISCORD_URL").expect("DISCORD_URL env var not set"))
            }),
        )
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .layer(tower_http::trace::TraceLayer::new_for_http());

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
