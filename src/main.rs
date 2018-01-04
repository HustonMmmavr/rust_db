extern crate iron;
extern crate router;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;
extern crate persistent;
extern crate params;
extern crate bodyparser;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
mod controllers {pub mod user; pub mod forum; pub mod post; pub mod thread;}

use iron::prelude::*;
use iron::status;
use router::Router;
use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use r2d2::{Pool, PooledConnection};
//use bodyparser;

#[macro_use]
mod db;
mod conf;

const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;

//use db;
//#[derive(Copy, Clone)]
//pub struct DbPool;
//impl iron::typemap::Key for DbPool {
//    type Value = r2d2::Pool<r2d2_postgres::PostgresConnectionManager>;
//}

fn fill_route(router: &mut Router) {//, db : &mut co) {
    // ------------------ user ----------------------------
    router.post("/api/user/:nickname/create",controllers::user::create_user, "user_create");
    router.get("/api/user/:nickname/profile", controllers::user::get_user, "get_user_profile");
    router.post("/api/user/:nickname/profile", controllers::user::update_user, "update_user");
//    // ------------------ forum ---------------------------
//    router.post("/api/forum/create", controllers::forum::create(), "forum_create");
//    router.post("/api/forum/:forum_slug/create", controllers::forum::create_thread, "create_thread");
//    router.get("/api/forum/:slug/details", controllers::forum::get(), "get_forum");
//    router.get("/api/forum/:slug/threads", controllers::forum::get_threads, "get_threads");
//    router.get("/api/forum/:slug/users", controllers::forum::get_users(), "get_users");
//    // ---------------- post ------------------------------
//    router.get("/api/post/:id/details", controllers::post::get_details, "get_details");
//    router.post("/api/post/:id/details", controllers::post::set_details, "set_details");
//    // ---------------  thread -----------------
    //router.get
}

//
//fn handler(req: &mut Request) -> IronResult<Response> {
//    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
//    Ok(Response::with((status::Ok, *query)))
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
    Iron::new(chain).http("localhost:5000").unwrap();

}

//    match db::get_pool(uri) {
//        Ok(pool) =>  chain.link(persistent::Read::<db::PostgresDB>::both(pool)),
//        Err(err) => panic!("postgres: {}", err),
//
//    }

//    Err(err) => {
//            panic!("postgres: {}", err);
////            std::process::exit(-1);
//        }

//    pool =
//        Err(err) => {
//            panic!("postgres: {}", err);
//            std::process::exit(-1);
//        }
//    };

//    chain.link(persistent::Read::<DbPool>::both(
//        pool));


//    chain.link_before(pool);
//    let pool = ::r2d2::Pool::new(manager).unwrap();


//pub fn get_pool(uri: &str) -> Result<PostgresPool, Box<Error>> {
//    let manager = try!(PostgresConnectionManager::new(uri, TlsMode::None));
////    let pool = ::r2d2::Pool::new(manager).unwrap();
//    let pool = try!(r2d2::Pool::new(manager));
//    Ok(pool)
//}

//extern crate iron;
//extern crate router;

//use iron::prelude::*;
//use iron::status;
//use router::Router;

//fn main() {
//    let mut router = Router::new();           // Alternative syntax:
//    router.get("/", handler, "index");        // let router = router!(index: get "/" => handler,
//    router.get("/:query", handler, "query");  //                      query: get "/:query" => handler);
//
//    Iron::new(router).http("localhost:3000").unwrap();
//
//    fn handler(req: &mut Request) -> IronResult<Response> {
//        let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
//        Ok(Response::with((status::Ok, *query)))
//

//    let manager = PostgresConnectionManager::new("postgres://mavr:951103@localhost/test",
//                                                 TlsMode::None).unwrap();
//    let manager = ::r2d2_postgres::PostgresConnectionManager::new(cn_str, ::postgres::SslMode::None).unwrap();
//    let manager = ::r2d2_postgres::PostgresConnectionManager::new(cn_str, ::postgres::SslMode::None).unwrap();

//    let config = ::r2d2::Conn::builder().pool_size(8).build();
//    ::r2d2::Pool::new(config, manager).unwrap()
//    let a = pool.max_size();
//    let b = "---------------------------------------";
//    println!("{}", b);
//    println!("{}", a.to_string());
//    let conn = pool.get();
//    let mut conn = pool.get().unwrap();
//    let mut chain = Chain::new(router);

//use r2d2_postgres::{PostgresConnectionManager;
//extern crate postgres;
//use controllers::user;
