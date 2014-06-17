#![crate_id = "sendfile"]
#![deny(missing_doc)]
#![feature(phase)]
#![feature(overloaded_calls)]

//! File-serving ingot for Iron

extern crate iron;
extern crate http;
#[phase(plugin, link)]
extern crate log;

use std::ops::Fn;
use std::io::IoResult;

use iron::{Ingot, Alloy, Request, Response};
use iron::ingot::{Status, Continue};

use http::status;

/// The SendFile `Ingot`.
#[deriving(Clone)]
pub struct SendFile {
    path: Option<Path>
}

impl SendFile {
    /// Create a new instance of the SendFile `Ingot`.
    pub fn new() -> SendFile {
        SendFile {
            path: None
        }
    }

    /// Specify the path to serve.
    pub fn path(&mut self, path: Path) -> Result<(), status::Status> {
        // Check for existence (fs::stat)
        // And permissions (fs::stat)
        // Before setting path
        self.path = Some(path);
        Ok(())
    }
}

impl Fn<(&'static mut Response,), IoResult<()>> for SendFile {
    /// Serve from specified path.
    fn call(&self, (_res,): (&mut Response,)) -> IoResult<()> {
        // Check that path exists (path has already checked for permissions)
        // Serve file
        Ok(())
    }
}

impl<'a, Rq: Request, Rs: Response> Ingot<Rq, Rs> for SendFile {
    /// Add the SendFile to the alloy.
    fn enter(&mut self, _req: &mut Rq, _res: &mut Rs, alloy: &mut Alloy) -> Status {
        alloy.insert::<SendFile>(self.clone());
        Continue
    }
}
