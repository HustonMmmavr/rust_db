extern crate iron;
extern crate router;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;
extern crate persistent;
mod controllers {pub mod user; pub mod forum; pub mod post; pub mod thread;}

use iron::prelude::*;
use iron::status;
use router::Router;
use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use r2d2::{Pool, PooledConnection};

#[macro_use]
mod db;
mod conf;

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


fn handler(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}


//pub fn get_pool(uri: &str) -> Result<PostgresPool, Box<Error>> {
//    let manager = try!(PostgresConnectionManager::new(uri, TlsMode::None));
////    let pool = ::r2d2::Pool::new(manager).unwrap();
//    let pool = try!(r2d2::Pool::new(manager));
//    Ok(pool)
//}

fn main() {

    let mut uri = "postgres://mavr:951103@localhost/test";
    let mut router = Router::new();           // Alternative syntax:
    fill_route(&mut router);
    let mut chain = Chain::new(router);

    let manager = (PostgresConnectionManager::new(uri, TlsMode::None)).unwrap();
//    let pool = ::r2d2::Pool::new(manager).unwrap();
    let pool = (r2d2::Pool::new(manager)).unwrap();

//    match db::get_pool(uri) {
//        Ok(pool) =>  chain.link(persistent::Read::<db::PostgresDB>::both(pool)),
//        Err(err) => panic!("postgres: {}", err),
//
//    }
    chain.link(persistent::Read::<conf::DbPool>::both(pool));
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
    Iron::new(chain).http("localhost:5000").unwrap();

}


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
//
//scope '/api' do
//scope '/user/:nickname' do
//post '/create',  :constraints => { :nickname => /[\w+\.]+/ },  to: 'user#create'
//get  '/profile', :constraints => { :nickname => /[\w+\.]+/ },   to: 'user#get'
//post '/profile', :constraints => { :nickname => /[\w+\.]+/ },   to: 'user#update'
//end
//
//scope '/forum' do
//post '/create', to: 'forum#create'
//post '/:forum_slug/create', to: 'forum#create_thread'
//scope '/:slug' do
//get '/details', to: 'forum#get_details'
//get '/threads', to: 'forum#get_threads'
//get '/users', to: 'forum#get_users'
//end
//end
//
//scope '/thread' do
//scope ':slug_or_id' do
//post '/create', to: 'thread#create_post'
//get '/details', to: 'thread#get_details'
//post '/details', to: 'thread#set_details'
//get '/posts', to: 'thread#get_posts'
//post '/vote', to: 'thread#vote'
//end
//end
//
//scope '/service' do
//get '/status', to: 'service#count'
//post '/clear', to: 'service#delete'
//end
//
//scope '/post' do
//scope '/:id' do
//get '/details', to: 'post#get_details'
//post '/details', to: 'post#set_details'
//end
//end