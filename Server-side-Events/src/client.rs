use futures::TryStreamExt;
use reqwest::{Client, StatusCode};

#[tokio::main]
async fn main() {
    let response = Client::new()
        .get("http://localhost:3000/sse")
        .send()
        .await
        .unwrap();

    if response.status() == StatusCode::OK {
        let mut stream = response.bytes_stream();

        while let Some(chunk) = stream.try_next().await.unwrap() {
            let chunk_string = std::str::from_utf8(&chunk).unwrap();
            println!("The chunk of data is: {:?}", chunk_string);
        }
    } else {
        // if an exception it will be here
        println!("The {:?}", response.text().await.unwrap())
    }
}
