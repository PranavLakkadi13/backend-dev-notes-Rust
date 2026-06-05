use std::{convert::Infallible, time::Duration};

use axum::{
    Router,
    response::{Sse, sse::Event},
    routing::get,
};
use chrono::Utc;
use futures::stream;
use tokio::net::TcpListener;
use tokio_stream::{Stream, StreamExt};

// the data being sent to the client
fn get_data() -> String {
    // returning the current time in the the below format
    format!(
        "The data is being sent from the server at : {:?}",
        Utc::now().format("%d/%m/%Y %H:%M %s").to_string()
    )
}

async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("The sse endpoint has been hit!!!!");
    // here we are creating a stream of the data and sending it through the Event, throttle is like a sleep method for duration
    let stream = stream::repeat_with(|| Event::default().data(get_data()))
        .map(Ok)
        .throttle(Duration::from_secs(5));
        // .take(6); This line is used when i want to kill the connection after n events

    // this is to keep the connection alive
    Sse::new(stream)
        .keep_alive(axum::response::sse::KeepAlive::new().interval(Duration::from_secs(5)))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/sse", get(sse_handler));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("The server is up and running at port: 3000");

    axum::serve(listener, app).await.unwrap();
}
