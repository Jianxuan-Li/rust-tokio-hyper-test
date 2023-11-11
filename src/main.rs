#![deny(warnings)]

use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

async fn hello(req: Request<Body>, addr: SocketAddr) -> Result<Response<Body>, Infallible> {
    println!("Hello World!");
    println!("addr: {}", addr);
    for (h_name, h_value) in req.headers() {
        println!(
            "{}: {}",
            h_name,
            h_value
                .to_str()
                .unwrap_or("Error: Non-visible ascii characters")
        );
    }
    println!("{}", req.uri());
    // print current time
    let now = chrono::Local::now();
    println!("----------{}", now.format("%Y-%m-%d %H:%M:%S"));

    // build json response, with current time, all headers in an array, and the uri
    let mut json = String::from("{\n");
    json.push_str(&format!(
        "\"time\": \"{}\",\n",
        now.format("%Y-%m-%d %H:%M:%S")
    ));
    json.push_str("\"headers\": [\n");
    for (h_name, h_value) in req.headers() {
        json.push_str(&format!(
            "  {{\"{}\": \"{}\"}},\n",
            h_name,
            h_value
                .to_str()
                .unwrap_or("Error: Non-visible ascii characters")
        ));
    }

    json.push_str("],\n");
    json.push_str(&format!("\"uri\": \"{}\"\n", req.uri()));

    // add ip address and port to json
    json.push_str(&format!(",\"ip\": \"{}:{}\"\n", addr.ip(), addr.port()));

    json.push_str("}\n");

    // set response headers to json
    let mut response = Response::new(Body::from(json));

    response.headers_mut().insert(
        hyper::header::CONTENT_TYPE,
        hyper::header::HeaderValue::from_static("application/json"),
    );

    // return response
    Ok(response)
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(move |conn: &AddrStream| {
        let addr = conn.remote_addr();
        async move {
            let addr = addr.clone();
            Ok::<_, Infallible>(service_fn(move |req| hello(req, addr.clone())))
        }
    });

    let addr = ([0, 0, 0, 0], 3000).into();

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
