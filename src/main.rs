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
extern crate iron_json_response as ijr;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
use ijr::{JsonResponseMiddleware, JsonResponse};

use iron::prelude::*;
use iron::status;
use router::Router;
use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use r2d2::{Pool, PooledConnection};
mod queries;
mod models;
mod managers;
//use std;
#[macro_use]
mod db;
mod conf;
mod controllers {pub mod user; pub mod forum; pub mod post; pub mod thread;}

const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;
fn fill_route(router: &mut Router) {
    // ------------------ user ----------------------------
    router.post("/api/user/:nickname/create",controllers::user::create_user, "user_create");
    router.get("/api/user/:nickname/profile", controllers::user::get_user, "get_user_profile");
    router.post("/api/user/:nickname/profile", controllers::user::update_user, "update_user");
//    // ------------------ forum ---------------------------
    router.post("/api/forum/create", controllers::forum::create, "forum_create");
    router.post("/api/forum/:slug/create", controllers::forum::create_thread, "create_thread");
    router.get("/api/forum/:slug/details", controllers::forum::get_forum, "get_forum");
    router.get("/api/forum/:slug/threads", controllers::forum::get_threads, "get_threads");
    router.get("/api/forum/:slug/users", controllers::forum::get_users, "get_users");
//    // ---------------- post ------------------------------
//    router.get("/api/post/:id/details", controllers::post::get_details, "get_details");
//    router.post("/api/post/:id/details", controllers::post::set_details, "set_details");
//    // ---------------  thread -----------------
    //router.get
}



//
//use postgres::{Connection, TlsMode};
//
//struct Person {
//    id: i32,
//    name: String,
//    data: Option<Vec<u8>>,
//}
//
//fn main() {
//    let conn = Connection::connect("postgres://mavr:951103@localhost:5432/test", TlsMode::None).unwrap();
//    let conn = Connection::connect("postgres://mavr:951103@localhost:5432/test", TlsMode::None).unwrap();
////    conn.execute("CREATE TABLE person (
////                    id              SERIAL PRIMARY KEY,
////                    name            VARCHAR NOT NULL,
////                    data            BYTEA
////                  )", &[]).unwrap();
//    let me = Person {
//        id: 0,
//        name: "Steven".to_string(),
//        data: None,
//    };
//    let b = conn.execute("Update person set name = 'b' WHERE name = 'Steven'",
//                 &[]).unwrap();
//    let query = &conn.query("SELECT id, name, data FROM person WHERE name='a'", &[]).unwrap();
//    println!("{}", b);
//    for row in &conn.query("SELECT id, name, data FROM person WHERE name='a'", &[]).unwrap() {
//        let person = Person {
//            id: row.get(0),
//            name: row.get(1),
//            data: row.get(2),
//        };
////        println!("Found person {}", person.name);
//    }
//
//}

fn main() {

    let mut uri = "postgres://mavr:951103@localhost/test1";
    let mut router = Router::new();           // Alternative syntax:
    fill_route(&mut router);
    let mut chain = Chain::new(router);

    let manager = (PostgresConnectionManager::new(uri, TlsMode::None)).unwrap();
    let pool = (r2d2::Pool::new(manager)).unwrap();

    chain.link_before(persistent::Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    chain.link(persistent::Read::<conf::DbPool>::both(pool));
    chain.link_after(JsonResponseMiddleware::new());
    Iron::new(chain).http("localhost:5000").unwrap();
}