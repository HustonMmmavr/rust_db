//mod user {
use conf::*;
use iron::prelude::*;
use iron;
use persistent;
//extern crate persistent;

    pub fn create_user(request : &mut Request) -> IronResult<Response> {
        let db_pool = &request.get::<persistent::Read<DbPool>>().unwrap();
        let conn = db_pool.get().unwrap();
        
        // get a connection from the pool

//        let mut conn = db_pool.get().unwrap();
        println!("Hello, world!");
        return Ok(Response::with((iron::status::Ok, "Hello World")));

    }


    pub fn get_user(request : &mut Request) -> IronResult<Response> {
        return Ok(Response::with((iron::status::Ok, "Hello World")));
    }

    pub fn update_user(request : &mut Request) -> IronResult<Response> {
        return Ok(Response::with((iron::status::Ok, "Hello World")));

    }

    pub fn count_user(request : &mut Request) -> IronResult<Response> {
        return Ok(Response::with((iron::status::Ok, "Hello World")));

    }

    fn clear_user(request : &mut Request) -> IronResult<Response> {
        return Ok(Response::with((iron::status::Ok, "Hello World")));

    }

//}