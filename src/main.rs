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
mod controllers {pub mod user; pub mod forum; pub mod post; pub mod thread; pub mod service;}

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
    router.post("/api/thread/:slug_or_id/create", controllers::thread::create_posts, "create_posts");
    router.get("/api/thread/:slug_or_id/details", controllers::thread::get_thread_, "get_thread");
    router.post("/api/thread/:slug_or_id/details", controllers::thread::update_thread_, "update_thread");
    router.post("/api/thread/:slug_or_id/vote", controllers::thread::vote_, "vote");
    router.get("/api/thread/:slug_or_id/posts", controllers::thread::get_posts, "get_posts");

    router.get("/api/post/:id/details", controllers::post::get_post_details, "get_post");
    router.post("/api/post/:id/details", controllers::post::set_post_details, "set_post");

    router.post("/api/service/clear", controllers::service::clear, "clear");
    router.get("/api/service/status", controllers::service::status, "status");


//    router.get("/api/forum/:slug/threads", controllers::forum::get_threads, "get_threads");

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
//    conn.prepare("IN");
////    conn.execute("CREATE TABLE person (
////                    id              SERIAL PRIMARY KEY,
////                    name            VARCHAR NOT NULL,
////                    data            BYTEA
////                  )", &[]).unwrap();
//}
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

//extern crate postgres;
extern crate postgres_binary_copy;
extern crate streaming_iterator;
//
//use postgres::{Connection, TlsMode};
//use postgres::types::{ToSql, INT4, VARCHAR, TIMESTAMPTZ, };
//use postgres_binary_copy::BinaryCopyReader;
//use streaming_iterator::StreamingIterator;
//
//fn main() {
//    let conn = Connection::connect("postgres://mavr:951103@localhost",
//                                   TlsMode::None).unwrap();
//
////    conn.execute("CREATE TABLE foo (id INT PRIMARY KEY, bar VARCHAR)", &[])
////        .unwrap();
//
//    let mut da: Vec<i32> = Vec::new();
//    da.push(1);
//    da.push(2);
//    let f: postgres_array::Array<i32> = postgres_array::Array::from_vec(da, 1);
//    println!("{:?}", f);
//    let post
//    let types = &[INT4, VARCHAR,];
//    let data: Vec<Box<ToSql>> = vec![Box::new(3i32), Box::new("hello"),
//                                     Box::new(4i32), Box::new("world")];
//    let data = streaming_iterator::convert(data.into_iter()).map_ref(|v| &**v);
//    let mut reader = BinaryCopyReader::new(types, data);
//
////    let stmt = conn.prepare("COPY aa (id, bar) FROM STDIN (FORMAT binary)").unwrap();
////    stmt.copy_in(&[], &mut reader).unwrap();
//}
//use std::thread;
////use r2d2_postgres::{TlsMode, PostgresConnectionManager};
////use r2d2::{Config};
//fn main() {
//    let manager = PostgresConnectionManager::new("postgres://comp:951103@localhost",
//                                                 TlsMode::None).unwrap();
////    let config = Config::builder()
////        .max_size(15)
////        .build(manager)
////        .unwrap();
//
//    let pool = r2d2::Pool::new(manager).unwrap();
//
//    for i in 0..1000000i32 {
//        let pool = pool.clone();
//        println!("{}", i);
//        thread::spawn(move || {
//            let conn = pool.get().unwrap();
//            conn.execute("INSERT INTO foo (bar) VALUES ($1)", &[&i]).unwrap();
//        });
//    }
//}
//
fn main() {
//    let mut v: Vec<_> = Vec::new();
//    v.push(5);
//    v.push("sa".to_string());
//    print!("{}", v);
//    String user = "comp"
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