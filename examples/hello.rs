#![feature(overloaded_calls)]
#![allow(unused_must_use)]
extern crate iron;
extern crate sendfile;
extern crate http;

use std::io::net::ip::Ipv4Addr;

use iron::{Iron, ServerT};

use sendfile::SendFile;


use iron::{Ingot, Alloy, Request, Response};
use iron::ingot::{Status, Continue};

#[deriving(Clone)]
struct HelloWorld;

impl HelloWorld {
  fn new() -> HelloWorld {
    HelloWorld
  }
}

impl<Rq: Request, Rs: Response> Ingot<Rq, Rs> for HelloWorld {
    fn enter(&mut self, _req: &mut Rq, res: &mut Rs, alloy: &mut Alloy) -> Status {
        let send_file = alloy.find_mut::<SendFile>().unwrap();
        send_file.path(Path::new("/hello.html"));
        (*send_file)(res);
        Continue
    }
}

fn main() {
    let mut server: ServerT = Iron::new();
    let send_file = SendFile::new();
    server.smelt(send_file);
    server.smelt(HelloWorld::new());
    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}
