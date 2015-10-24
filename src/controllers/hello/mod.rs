use iron::{IronResult, Request, Response};
use iron::status;
use router::Router;
use plugin::Pluggable;

use super::{json_empty_error, json_response, json_response_code, nested_str, RequestHelper};
use models::hello::Hello;

pub fn router(router: &mut Router) {
    router.get("/hello", index);
    router.get("/hello/:id", get);
    router.post("/hello", create);
    router.delete("/hello/:id", delete);
}

pub fn index(request: &mut Request) -> IronResult<Response> {
    let hellos = Hello::all(request.db());

    json_response(status::Ok, &hellos)
}

pub fn get(request: &mut Request) -> IronResult<Response> {
    let id = request.find::<i32>("id");
    if id.is_none() { return json_empty_error(status::BadRequest); }

    let hello = Hello::get(id.unwrap(), request.db());
    if hello.is_none() { return json_empty_error(status::UnprocessableEntity); }

    json_response(status::Ok, &hello.unwrap())
}

pub fn create(request: &mut Request) -> IronResult<Response> {
    match request.json_body() {
        Some(body) => {
            let mut hello = Hello::new(nested_str(&body, vec!["content"]).unwrap());
            match hello.create(request.db()) {
                Some(_) => json_response_code(status::Created),
                None => json_empty_error(status::UnprocessableEntity)
            }
        }
        None => json_empty_error(status::BadRequest)
    }
}

pub fn delete(request: &mut Request) -> IronResult<Response> {
    let id = request.find::<i32>("id");

    json_response_code(match id {
        Some(id) => {
            if Hello::delete(id, request.db()) {
                status::NoContent
            } else {
                status::UnprocessableEntity
            }
        }
        None => status::BadRequest
    })
}
