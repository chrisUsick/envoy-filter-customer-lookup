#![deny(warnings)]
use serde::{Deserialize, Serialize};
use warp::{Filter, reply::Json};

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("api" /  "lookup")
        .and(warp::header("Authorization"))
        .map(lookup);

    warp::serve(hello)
        .run(([0, 0, 0, 0], 3030))
        .await;
}

#[derive(Deserialize, Serialize)]
struct Customer {
    id: u32,
    name: String,
}

fn lookup(token: String) -> Json {
    println!("token: {}", token);
    let cust = Customer{id: 1, name: String::from("Acme Co.")};
    warp::reply::json(&cust)
}
