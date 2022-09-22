use serde_derive::Deserialize;
use std::collections::HashMap;
use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /ping => 200 OK with body "{"version":"0.1.0"}"
    let route_health = warp::get()
        .and(warp::path("health"))
        .and(warp::path::end())
        .map(|| {
            let mut resp = HashMap::new();
            resp.insert("version", "0.1.0");
            warp::reply::json(&resp)
        });

    // POST /set {"key" : "xx", "value" : "xx"} => 200 OK with body "{"key" : "xx", "value" : "xx"}"
    let route_set = warp::post()
        .and(warp::path("set"))
        .and(warp::path::end())
        .and(warp::body::json())
        .map(|entry: Entry| {
            let mut resp = HashMap::new();
            resp.insert("key", entry.key);
            resp.insert("value", entry.value);
            warp::reply::json(&resp)
        });

    // GET /get/key => 200 OK with body "{"key" : "xx", "value" : "xx"}"
    let route_get = warp::get()
        .and(warp::path("get"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .map(|key: String| {
            let mut resp = HashMap::new();
            resp.insert("key", key);
            resp.insert("value", "demo".to_string());
            warp::reply::json(&resp)
        });

    // ROUTES
    let routes = route_health
        .or(route_set)
        .or(route_get)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

#[derive(Deserialize)]
pub struct Entry {
    key: String,
    value: String,
}
