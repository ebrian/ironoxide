extern crate hyper;
extern crate iox;
extern crate app;

use std::io::Write;

use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;
use hyper::status::StatusCode;
use hyper::net::Fresh;
use hyper::uri::RequestUri;

//use library::iron_oxide::math;

use app::controllers::index_controller::index;

fn main() {
    Server::http(|req: Request, mut res: Response| {
        *res.status_mut() = match (req.method, req.uri) {
            (hyper::Get, RequestUri::AbsolutePath(ref path)) if path == "/"  => {
                StatusCode::Ok
            },
            (hyper::Get, _) => StatusCode::NotFound,
            _ => StatusCode::MethodNotAllowed
        };

        match req.uri {
            RequestUri::AbsolutePath("/") => {
                res.write_all(b"Hello World!").unwrap();
                res.end().unwrap();
            }
        }
    })
        .listen("127.0.0.1:8080")
        .unwrap();
}

fn bootstrap(req: Request, res: Response<Fresh>) {
    println!("remote: {}", req.remote_addr);
    println!("method: {}", req.method);
    println!("uri: {:?}", req.uri);
    println!("headers: {}", req.headers);
    println!("version: {}", req.version);

    //let uri_string: String = req.uri;

    //iox::test::test::hello();

    if req.uri == RequestUri::AbsolutePath("/".to_owned()) {
        println!("slash");
    }


    //let name = "index_controller";
    //app::controllers::name.to_string()::index::index();

    //for i in req.body {
    //    println!("{:?}", i);
    //}
    let mut res = res.start().unwrap();
    res.write_all(b"Hello World!").unwrap();
    res.end().unwrap();
}
