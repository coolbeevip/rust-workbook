#![feature(future_join)]

use http_warp::http_service::HttpService;
use std::future::Future;
use std::net::SocketAddr;
use tokio::sync::oneshot;

mod http_service;

#[test]
fn test() {
    let (tx, rx) = oneshot::channel();
    assert_eq!(1, 1);
}
