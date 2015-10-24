extern crate iron;
extern crate bodyparser;
extern crate persistent;
extern crate plugin;
extern crate router;

extern crate rusqlite;
extern crate time;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate r2d2;
extern crate r2d2_sqlite;

extern crate rustc_serialize;

use iron::Iron;
use iron::middleware::Chain;

use std::net::Ipv4Addr;

mod controllers;
mod models;

fn run_server(port: u16) {
    let addr = Ipv4Addr::new(127, 0, 0, 1);
    let router = controllers::router();

    let mut middleware = Chain::new(router);
    middleware = models::database_middleware(middleware);
    middleware = controllers::body_middleware(middleware);

    match Iron::new(middleware).http((addr, port)) {
        Ok(server) => info!("Listening on {:?}...", server.socket.port()),
        Err(why) => error!("Could not start server: {:?}", why)
    }
}

fn main() {
    let _ = env_logger::init();

    run_server(8080)
}
