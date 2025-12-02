use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{Router, routing::post};
use tokio::net::TcpListener;

use crate::{db::DB, query_engine::Query};

mod db;
mod query_engine;
mod user_service;

async fn graphql_handler(request: GraphQLRequest) -> GraphQLResponse {
    let query = Query { db: DB };

    let schema = Schema::new(query, EmptyMutation, EmptySubscription);

    let res = schema.execute(request.into_inner()).await;

    res.into()
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/gql", post(graphql_handler));

    let listener = TcpListener::bind("0.0.0.0:30000").await.unwrap();

    println!("Server is running....");

    axum::serve(listener, app).await.unwrap();
}
