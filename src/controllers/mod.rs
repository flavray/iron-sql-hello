pub mod hello;

use iron::{IronResult, Request, Response};
use iron::middleware::Chain;
use iron::status::Status;

use bodyparser;
use persistent::Read;
use plugin::Pluggable;
use router::Router;

use rustc_serialize::{json, Encodable};

use std::str::FromStr;

use models::{AppDb, SqlPooledConnection};

pub fn router() -> Router {
    let mut router = Router::new();
    hello::router(&mut router);

    router
}

pub trait RequestHelper {
    // shorthand to ease database connection access from controllers
    fn db(&self) -> SqlPooledConnection;

    // shorthand to ease query parameter access from controllers
    fn find<T: FromStr>(&self, parameter: &str) -> Option<T>;

    // shorthand to ease JSON body access from controllers
    fn json_body(&mut self) -> Option<json::Json>;
}

impl<'a, 'b> RequestHelper for Request<'a, 'b> {
    fn db(&self) -> SqlPooledConnection {
        let pool = self.extensions.get::<Read<AppDb>>().unwrap();
        pool.get().unwrap()
    }

    fn find<T: FromStr>(&self, parameter: &str) -> Option<T> {
        let value = self.extensions.get::<Router>().unwrap().find(parameter);
        if value.is_none() { return None; }

        match value.unwrap().parse::<T>() {
            Ok(i) => Some(i),
            Err(_) => None
        }
    }

    fn json_body(&mut self) -> Option<json::Json> {
        match self.get::<bodyparser::Json>() {
            Ok(body) => body,
            Err(_) => None
        }
    }
}

pub fn body_middleware(mut middleware: Chain) -> Chain {
    let max_body_length: usize = 1024 * 1024 * 10;
    middleware.link_before(Read::<bodyparser::MaxBodyLength>::one(max_body_length));
    middleware
}

// return a string within a JSON tree, following items in reverse order
pub fn nested_str(body: &json::Json, mut items: Vec<&str>) -> Option<String> {
    if items.len() == 0 {
        return match body.as_string() {
            Some(b) => Some(b.to_string()),
            None => None
        }
    }

    let item = items[items.len()-1];
    match body.find(item) {
        Some(current) => {
            items.pop();
            nested_str(current, items)
        }
        None => None
    }
}

pub fn json_response<T: Encodable>(code: Status, data: &T) -> IronResult<Response> {
    let string = json::encode(data).unwrap();
    Ok(Response::with((code, string)))
}

pub fn json_response_code(code: Status) -> IronResult<Response> {
    Ok(Response::with(code))
}

pub fn json_empty_error(code: Status) -> IronResult<Response> {
    Ok(Response::with((code, "{}")))
}
