use http_server_tiny::{HttpServer, Method, Res};
use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = HttpServer::new("0.0.0.0:8000", "error.html");
    server.add_route(
        &Method::Get,
        "/",
        Box::new(|_| Res::File {
            name: "./static/index.html",
            ct: "text/html; charset=utf-8",
            sc: 200,
        }),
    );
    server.add_route(
        &Method::Get,
        "/hi",
        Box::new(|_| Res::File {
            name: "./static/hi.html",
            ct: "text/html; charset=utf-8",
            sc: 200,
        }),
    );

    let server = Arc::new(Mutex::new(server));

    let mut handles = Vec::new();

    for _ in 0..4 {
        let server = server.clone();
        let handle = thread::spawn(move || {
            server
                .lock()
                .unwrap()
                .handle_requests(Box::new(|req_in| {
                    println!("INFO: {} '{}'\n", req_in.method, req_in.url);
                }))
                .unwrap();
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}
