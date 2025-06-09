//! Default Compute template program.

use fastly::backend::BackendBuilder;
use fastly::http::{header, Method, StatusCode};
use fastly::{mime, Error, Request, Response};

use schemars::JsonSchema;
use serde::Serialize;
use serde::Deserialize;


#[derive(Serialize, JsonSchema)]
struct Pong {
    ping: String
}

#[derive(Deserialize, Serialize, JsonSchema)]
struct HelloWorld {
    world: String
}


/// The entry point for your application.
///
/// This function is triggered when your service receives a client request. It could be used to
/// route based on the request properties (such as method or path), send the request to a backend,
/// make completely new requests, and/or generate synthetic responses.
///
/// If `main` returns an error, a 500 error response will be delivered to the client.
#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    let build_stamp = option_env!("TIMESTAMP").unwrap_or("00000000T000000Z");
    let fastly_service_version = std::env::var("FASTLY_SERVICE_VERSION")
        .unwrap_or_else(|_| String::new());

    // Log service version
    println!(
        "FASTLY_SERVICE_VERSION: {}",
        std::env::var("FASTLY_SERVICE_VERSION").unwrap_or_else(|_| String::new())
    );

    println!("Build time: {}", build_stamp);

    let pooled_backend = BackendBuilder::new("pooled", "zonena.me")
    // pooled_backend
        // .enable_ssl()
        // .check_certificate("zonena.me")
        .enable_pooling(true)
        .finish()?;

    let div_backend = BackendBuilder::new("divided", "zonename.org")
    // div_backend
        // .enable_ssl()
        // .check_certificate("zonena.me")
        .enable_pooling(false)
        .finish()?;

    // Filter request methods...
    match req.get_method() {
        // Block requests with unexpected methods
        &Method::POST | &Method::PUT | &Method::PATCH | &Method::DELETE => {
            return Ok(Response::from_status(StatusCode::METHOD_NOT_ALLOWED)
                .with_header(header::ALLOW, "GET, HEAD, PURGE")
                .with_body_text_plain("This method is not allowed\n"))
        }

        // Let any other requests through
        _ => (),
    };

    // Pattern match on the path...
    match req.get_path() {
        // If request is to the `/` path...
        "/" => {
            // Below are some common patterns for Compute services using Rust.
            // Head to https://developer.fastly.com/learning/compute/rust/ to discover more.

            // Create a new request.
            // let mut bereq = Request::get("http://httpbin.org/headers")
            //     .with_header("X-Custom-Header", "Welcome to Compute!")
            //     .with_ttl(60);

            // Add request headers.
            // bereq.set_header(
            //     "X-Another-Custom-Header",
            //     "Recommended reading: https://developer.fastly.com/learning/compute",
            // );

            // Forward the request to a backend.
            // let mut beresp = bereq.send("backend_name")?;

            // Remove response headers.
            // beresp.remove_header("X-Another-Custom-Header");

            // Log to a Fastly endpoint.
            // use std::io::Write;
            // let mut endpoint = fastly::log::Endpoint::from_name("my_endpoint");
            // writeln!(endpoint, "Hello from the edge!").unwrap();

            // Send a default synthetic response.
            Ok(Response::from_status(StatusCode::OK)
                .with_header("X-Fastly-Service_version", fastly_service_version)
                .with_header("X-Build-Stamp", build_stamp)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_body(include_str!("welcome-to-compute.html")))
        }

        "/ping" => {
            let pong = Pong {
                ping: String::from("pong"),
            };
            Ok(Response::from_status(StatusCode::OK)
                .with_header("X-Fastly-Service_version", fastly_service_version)
                .with_header("X-Build-Stamp", build_stamp)
                .with_body_json(&pong)?)
        }

        "/pooled" => {
            let mut pooled_resp = req.send(pooled_backend)
                .expect("request succeeds");
            pooled_resp
                .set_header("X-Fastly-Service_version", fastly_service_version);
            pooled_resp
                .set_header("X-Build-Stamp", build_stamp);
            Ok(pooled_resp)
        }

        "/div" => {
            let mut div_resp = req.send(div_backend)
                .expect("request succeeds");
            div_resp
                .set_header("X-Fastly-Service_version", fastly_service_version);
            div_resp
                .set_header("X-Build-Stamp", build_stamp);
            Ok(div_resp)
        }

        // Catch all other requests and return a 404.
        _ => Ok(Response::from_status(StatusCode::NOT_FOUND)
            .with_body_text_plain("The page you requested could not be found\n")),
    }
}
