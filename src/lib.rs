use std::{collections::HashMap, fs::File, net::ToSocketAddrs};
pub use tiny_http::Method;
use tiny_http::{Header, Request, Response, Server};

pub fn handle_file<'a>(file_name: &'a str, ct: &'a str, sc: u32) -> Response<File> {
    Response::from_file(File::open(file_name).unwrap())
        .with_header(Header::from_bytes("Content-Type", ct).unwrap())
        .with_status_code(sc)
}

pub struct RequestInside<'a> {
    pub header: &'a [Header],
    pub method: &'a Method,
    pub content: String,
    pub url: &'a str,
}

pub enum Res<'a> {
    Json(String),
    File { name: &'a str, ct: &'a str, sc: u32 },
    Empty,
}

impl<'a> RequestInside<'a> {
    fn new(req: &'a mut Request) -> Self {
        let mut content = String::new();
        req.as_reader().read_to_string(&mut content).unwrap();
        Self {
            header: req.headers(),
            content,
            method: req.method(),
            url: req.url(),
        }
    }
}

type Func<'a> = Box<dyn Fn(RequestInside) -> Res<'a> + Send + 'static + Sync>;

pub struct HttpServer<'a> {
    server: Server,
    error_path: &'a str,
    pub routes: HashMap<(&'a Method, &'a str), Func<'a>>,
}

impl<'a> HttpServer<'a> {
    pub fn new<S>(addr: S, error_path: &'a str) -> Self
    where
        S: ToSocketAddrs,
    {
        Self {
            server: Server::http(addr).unwrap(),
            error_path,
            routes: HashMap::new(),
        }
    }
    pub fn add_route(&mut self, method: &'a Method, url: &'a str, f: Func<'a>) {
        self.routes.insert((method, url), f);
    }

    pub fn handle_requests(
        &self,
        f: Box<dyn Fn(&RequestInside) -> ()>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for mut request in self.server.incoming_requests() {
            let req = RequestInside::new(&mut request);
            let result = self.routes.get(&(req.method, req.url));

            f(&req);

            if let Some(func) = result {
                let res = func(req);
                let _ = match res {
                    Res::Json(content) => request.respond(
                        Response::from_string(content)
                            .with_status_code(200)
                            .with_header(
                                Header::from_bytes("Content-Type", "application/json").unwrap(),
                            ),
                    ),
                    Res::File { name, ct, sc } => request.respond(handle_file(name, ct, sc)),
                    Res::Empty => request.respond(Response::empty(200)),
                };
            } else {
                request.respond(handle_file(
                    self.error_path,
                    "text/html; charset=utf-8",
                    404,
                ))?;
            }
        }

        Ok(())
    }
}
