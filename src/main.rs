extern crate hyper;
extern crate iox;

use std::io::Write;

use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;
use hyper::net::Fresh;

//use library::iron_oxide::math;

fn main() {
    Server::http(bootstrap)
        .listen("127.0.0.1:8080")
        .unwrap();
}

fn bootstrap(req: Request, res: Response<Fresh>) {
    println!("remote: {}", req.remote_addr);
    println!("method: {}", req.method);
    println!("uri: {:?}", req.uri);
    println!("headers: {}", req.headers);
    println!("version: {}", req.version);

    iox::test::test::hello();

    //for i in req.body {
    //    println!("{:?}", i);
    //}
    let mut res = res.start().unwrap();
    res.write_all(b"Hello World!").unwrap();
    res.end().unwrap();
}
