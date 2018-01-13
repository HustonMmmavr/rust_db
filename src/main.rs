extern crate iron;
extern crate router;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;
extern crate persistent;
extern crate params;
extern crate bodyparser;
extern crate time;
extern crate chrono;
extern crate postgres_array;
extern crate iron_json_response as ijr;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate flame;
extern crate postgres_binary_copy;
extern crate streaming_iterator;
extern crate hyper;
//use hyper//::http::message::Protocol;

//use hyper::header::{Protocol};
use std::net::TcpListener;
use hyper::net::NetworkListener;
use std::net::SocketAddr;
use hyper::net::HttpStream;
//use hyper:;
use std::io;
use std::sync::Arc;
use ijr::{JsonResponseMiddleware, JsonResponse};

use iron::prelude::*;
use iron::status;
use router::Router;
use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use r2d2::{Pool, PooledConnection};
mod queries;
mod models;
mod managers;
#[macro_use]
mod db;
mod conf;

mod controllers {pub mod user; pub mod forum; pub mod post; pub mod thread; pub mod service;}

const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;

const USER: &'static str = "mavr";
const PASSWORD: &'static str = "951103";


fn fill_route(router: &mut Router) {
    // ------------------ user ----------------------------
    router.post("/api/user/:nickname/create",controllers::user::create_user, "user_create");
    router.get("/api/user/:nickname/profile", controllers::user::get_user, "get_user_profile");
    router.post("/api/user/:nickname/profile", controllers::user::update_user, "update_user");
    // ------------------ forum ---------------------------
    router.post("/api/forum/create", controllers::forum::create, "forum_create");
    router.post("/api/forum/:slug/create", controllers::forum::create_thread, "create_thread");
    router.get("/api/forum/:slug/details", controllers::forum::get_forum, "get_forum");
    router.get("/api/forum/:slug/threads", controllers::forum::get_threads, "get_threads");
    router.get("/api/forum/:slug/users", controllers::forum::get_users, "get_users");
    // ---------------- thread ------------------------------
    router.post("/api/thread/:slug_or_id/create", controllers::thread::create_posts, "create_posts");
    router.get("/api/thread/:slug_or_id/details", controllers::thread::get_thread_, "get_thread");
    router.post("/api/thread/:slug_or_id/details", controllers::thread::update_thread_, "update_thread");
    router.post("/api/thread/:slug_or_id/vote", controllers::thread::vote_, "vote");
    router.get("/api/thread/:slug_or_id/posts", controllers::thread::get_posts, "get_posts");
    // ---------------- post ------------------------------
    router.get("/api/post/:id/details", controllers::post::get_post_details, "get_post");
    router.post("/api/post/:id/details", controllers::post::set_post_details, "set_post");
    // ---------------- service ----------------------------
    router.post("/api/service/clear", controllers::service::clear, "clear");
    router.get("/api/service/status", controllers::service::status, "status");
}

//
#[derive(Clone)]
struct TcpListenerNoDelay {
    listener: Arc<TcpListener>,
}

impl NetworkListener for TcpListenerNoDelay {
    type Stream = HttpStream;

    fn accept(&mut self) -> Result<Self::Stream, hyper::Error> {
        let tcp = try!(self.listener.accept());
        try!(tcp.0.set_nodelay(true));
        let stream = HttpStream(tcp.0);
        Ok(stream)
    }

    fn local_addr(&mut self) -> io::Result<SocketAddr> {
        self.listener.local_addr()
    }
}


use hyper::Error;
fn main() {

    let uri = "postgres://mavr:951103@0.0.0.0:5432/test";
    let mut router = Router::new();           // Alternative syntax:
    fill_route(&mut router);
    let mut chain = Chain::new(router);

    let manager;
    match (PostgresConnectionManager::new(uri, TlsMode::None)) {
        Ok(val) => manager = val,
        Err(e) => panic!("Error db {:?}", e)
    }
    let pool = (r2d2::Pool::new(manager)).unwrap();

    chain.link_before(persistent::Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    chain.link(persistent::Read::<conf::DbPool>::both(pool));
    chain.link_after(JsonResponseMiddleware::new());
    let listener;
    match  TcpListener::bind("0.0.0.0:5001") {
        Ok(val) => listener = val,
        Err(e) => panic!("Error listen {:?}", e),
    }

    println!("here");
    Iron::new(chain).listen(TcpListenerNoDelay { listener: Arc::new(listener) },
                               iron::Protocol::http()).unwrap();
}