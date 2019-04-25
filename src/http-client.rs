use futures::Future;
use hyper::{
    client::{connect::Destination, HttpConnector},
    Request, Response, Uri,
};
use std::time::Duration;
use tower::{MakeService, Service};
use tower::builder::ServiceBuilder;
use tower_hyper::{
    Body,
    client::{Builder, Connect},
    util::Connector,
};

fn main() {
    let fut = futures::lazy(|| {
        request().map(|resp| {
            dbg!(resp);
        })
    });
    hyper::rt::run(fut)
}

fn request() -> impl Future<Item = Response<hyper::Body>, Error = ()> {
    let connector = Connector::new(HttpConnector::new(1));
    let hyper = Connect::new(connector, Builder::new());

    // We're calling the tower/examples/server.rs.
    let dst = Destination::try_from_uri(Uri::from_static("http://127.0.0.1:3000")).unwrap();

    // Now, to build the service!
    let mut maker = ServiceBuilder::new()
        .buffer(5)
        .rate_limit(5, Duration::from_secs(1))
        .concurrency_limit(5)
        .service(hyper);

    let client = maker
        .make_service(dst)
        .map_err(|err| eprintln!("Connect Error {:?}", err));

    let request = Request::builder()
        .method("GET")
        .body(Vec::new())
        .unwrap();

    client
        .map_err(|e| panic!("Service is not ready: {:?}", e))
        .and_then(|mut c| {
            c.call(request)
                .map(|res| res.map(Body::into_inner))
                .map_err(|e| panic!("{:?}", e))
        })
}
