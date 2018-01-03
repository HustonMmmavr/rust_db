extern crate iron;
extern crate router;
//mod rust_db;
//extern module_path!("./controllers/user.rs");
use iron::prelude::*;
use iron::status;
use router::Router;
mod controllers {pub mod user; }
use controllers::user;
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

fn fill_route(router: &mut Router) {
    // ------------------ user ----------------------------
    router.post("/api/user/:nickname/create", controllers::user::create_user, "han");
    router.get("/api/user/:nickname/profile", handler, "han");
    router.post("/api/user/:nickname/profile", handler, "han");
    // ------------------ forum ---------------------------


}


fn handler(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

fn main() {
    let mut router = Router::new();           // Alternative syntax:
    fill_route(&mut router);
//    router.post("/api/user/create", handler, "index");        // let router = router!(index: get "/" => handler,
//    router.get("/:query", handler, "query");  //                      query: get "/:query" => handler);
//
    Iron::new(router).http("localhost:5000").unwrap();

}

////fn server() {
////
////}
////fn server() {
////
////}
//
//extern crate hyper;
//extern crate futures;
//
//use futures::future::Future;
//
//use hyper::header::ContentLength;
//use hyper::server::{Http, Request, Response, Service};
//
//struct HelloWorld;
//const PHRASE: &'static str = "Hello, World!";
//
//impl Service for HelloWorld {
//    // boilerplate hooking up hyper's server types
//    type Request = Request;
//    type Response = Response;
//    type Error = hyper::Error;
//    // The future representing the eventual Response your call will
//    // resolve to. This can change to whatever Future you need.
//    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;
//
//    fn call(&self, _req: Request) -> Self::Future {
//        // We're currently ignoring the Request
//        // And returning an 'ok' Future, which means it's ready
//        // immediately, and build a Response with the 'PHRASE' body.
//        Box::new(futures::future::ok(
//            Response::new()
//                .with_header(ContentLength(PHRASE.len() as u64))
//                .with_body(PHRASE)
//        ))
//    }
//}
//
//fn main() {
//    let addr = "127.0.0.1:5000".parse().unwrap();
//    let server = Http::new().bind(&addr, || Ok(HelloWorld)).unwrap();
//    server.run().unwrap();
////    println!("Hello, world!");
//}
