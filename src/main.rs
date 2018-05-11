extern crate env_logger;
extern crate http;
extern crate simple_server;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;
use http::header;
use simple_server::{Method, Server, StatusCode};
use std::path::PathBuf;
use std::thread;

fn main() {
    env_logger::init().unwrap();

    println!("hello");

    let t = thread::spawn(move || {
        let host = "127.0.0.1";
        let port = "7878";

        let server = Server::new(|request, mut response| {
            info!("Request received. {} {}", request.method(), request.uri());

            match (request.method(), request.uri().path()) {
                (&Method::POST, "/get-work") => {
                    response.header(header::CONTENT_TYPE, "application/json".as_bytes());

                    let files = get_relevant_files()?;

                    let body = json!({
                        "files": files,
                    });
                    let body = match serde_json::to_vec(&body) {
                        Ok(bytes) => bytes,
                        Err(_) => {
                            response.status(StatusCode::INTERNAL_SERVER_ERROR);
                            r#"{"error": true}"#.into()
                        }
                    };
                    Ok(response.body(body)?)
                }
                (_, _) => {
                    response.status(StatusCode::NOT_FOUND);
                    Ok(response.body(
                        r#"{"error": true, "message": "Not found"}"#.as_bytes()
                            .to_vec(),
                    )?)
                }
            }
        });

        server.listen(host, port);
    });

    t.join();
}

fn get_relevant_files() -> Result<Vec<PathBuf>, ::std::io::Error> {
    use std::fs::{self, DirEntry};

    let dir = "./test/files";
    let mut res = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            res.push(path);
        }
    }

    Ok(res)
}
