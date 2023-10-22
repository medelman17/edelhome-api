use async_graphql::http::GraphiQLSource;
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema, SimpleObject, ID};
use async_graphql_poem::GraphQL;
use poem::{get, handler, listener::TcpListener, web::Html, IntoResponse, Route, Server};

use hue::{Hue, QueryRoot};

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    let app = Route::new().at("/", get(graphiql).post(GraphQL::new(schema)));

    Server::new(TcpListener::bind("127.0.0.1:4003"))
        .run(app)
        .await
}
