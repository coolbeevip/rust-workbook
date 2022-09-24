use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::time::Duration;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use tokio::sync::oneshot;
use warp::http::StatusCode;
use warp::Filter;

use log::{error, info};
use tokio::time;

// 定义结构体
pub struct HttpService {
    addr: SocketAddr,
}

// 定义结构体方法
//#[async_trait::async_trait]
impl HttpService {
    pub fn new(addr: SocketAddr) -> HttpService {
        HttpService { addr }
    }

    // GET /ping => 200 OK with body "{"version":"0.1.0"}"
    fn route_health(
        &self,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::get()
            .and(warp::path("health"))
            .and(warp::path::end())
            .map(|| {
                let mut resp = HashMap::new();
                resp.insert("version", "0.1.0");
                warp::reply::json(&resp)
            })
    }

    // POST /set {"key" : "xx", "value" : "xx"} => 200 OK with body "{"key" : "xx", "value" : "xx"}"
    fn route_set(
        &self,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::post()
            .and(warp::path("set"))
            .and(warp::path::end())
            .and(warp::body::json())
            .map(|entry: Entry| {
                let mut resp = HashMap::new();
                resp.insert("key", entry.key);
                resp.insert("value", entry.value);
                warp::reply::json(&resp)
            })
    }

    // GET /get/key => 200 OK with body "{"key" : "xx", "value" : "xx"}"
    fn route_get(
        &self,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::get()
            .and(warp::path("get"))
            .and(warp::path::param::<String>())
            .and(warp::path::end())
            .map(|key: String| {
                let mut resp = HashMap::new();
                resp.insert("key", key);
                resp.insert("value", "demo".to_string());
                warp::reply::json(&resp)
            })
    }

    // ROUTES
    fn routes(&self) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        self.route_health()
            .or(self.route_set())
            .or(self.route_get())
            .with(warp::cors().allow_any_origin())
    }

    pub fn start(&self) -> std::io::Result<()> {
        let routes = self.routes().recover(handle_rejection);
        let (tx, rx) = oneshot::channel();
        let (_addr, server) = warp::serve(routes).bind_with_graceful_shutdown(self.addr, async {
            rx.await.ok();
        });
        info!("Server initialized with : {}", _addr);
        tokio::task::spawn(server);
        //awaitility::at_most(Duration::from_secs(60)).until(|| false);
        let _ = tx.send(());
        // tokio::runtime::Builder::new_multi_thread()
        //     .enable_all()
        //     .build()?
        //     .block_on(async {
        //         warp::serve(self.routes()).run(([0, 0, 0, 0], 3030)).await;
        //     });

        Ok(())

        // warp::serve(self.routes()).run(([127, 0, 0, 1], 3030)).await;
        //let (tx, rx) = oneshot::channel();
        // warp::serve(self.routes()).bind_with_graceful_shutdown(self.addr, async {
        //     //rx.await.ok();
        //     info!("http server graceful shutdown!");
        // });
    }
}

// 异常处理
async fn handle_rejection(rejection: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    error!("handle error: {:?}", rejection);

    let code = StatusCode::INTERNAL_SERVER_ERROR;
    let message = format!("TODO Wrap Error: {:?}", rejection);

    let json = warp::reply::json(&ErrorResponse {
        code: code.as_u16(),
        message,
    });

    Ok(warp::reply::with_status(json, code))
}

#[derive(Deserialize)]
pub struct Entry {
    key: String,
    value: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
}
