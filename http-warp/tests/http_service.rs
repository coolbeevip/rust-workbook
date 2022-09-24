#![feature(future_join)]

#[cfg(test)]
mod tests {
    use futures::executor::block_on;
    use http_warp::http_service::HttpService;
    use std::future::join;
    use std::net::SocketAddr;
    use std::thread;
    use std::time::Duration;

    #[tokio::test]
    async fn test() {
        let addr = "127.0.0.1:3030"
            .parse::<SocketAddr>()
            .expect("Invalid grpc_host");
        let http = HttpService::new(addr);
        let future = http.start();
        //future.await
        awaitility::at_most(Duration::from_secs(60)).until(|| false);
    }

    #[test]
    fn test3() {
        env_logger::init();
        let addr = "127.0.0.1:3032"
            .parse::<SocketAddr>()
            .expect("Invalid grpc_host");
        let http = HttpService::new(addr);
        let future = http.start();
        // let mut rt = tokio::runtime::Runtime::new().unwrap();
        // rt.block_on(async {
        //     future.await
        // });
        println!("ok");
        awaitility::at_most(Duration::from_secs(60)).until(|| false);
    }

    #[test]
    fn test2() {
        let addr = "127.0.0.1:3032"
            .parse::<SocketAddr>()
            .expect("Invalid grpc_host");
        let http = HttpService::new(addr);

        // thread::spawn(|| async move{
        //     let future = http.start();
        //     future.await
        // });

        awaitility::at_most(Duration::from_secs(60)).until(|| false);
        println!("ok")
        // warp::test::request().path("/health").filter()
        //block_on(future);
    }
}
