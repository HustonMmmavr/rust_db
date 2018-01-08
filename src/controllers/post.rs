use conf::*;
use iron::prelude::*;
use iron;
use persistent;
use bodyparser;
use persistent::Read;
use iron::status;
use router::Router;
use std::vec::Vec;
use std::string::String;
use std::str::FromStr;
use serde_json::Error;
use postgres::types::ToSql;
use postgres::rows::Row;
#[macro_use]
use serde_derive;
use serde_json;
use params::{Params, Value};
use params;
use std::io::copy;
use ijr;
use db;
use ijr::{JsonResponseMiddleware, JsonResponse};


pub fn get_post_details(request: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    let db_pool = &request.get::<persistent::Read<DbPool>>().unwrap();
    let conn = db_pool.get().unwrap();
    let related = request.get::<Params>().unwrap();
    let id:i32 = serde_json::from_str(request.extensions.get::<Router>().unwrap().find("id").unwrap()).unwrap();

//    match get_post(id, related, &conn) {
//
//    }

//    let mut string: String = String::new();
//    match data.find(&["related"]) {
//        Some(val) => string = params::FromValue::from_value(val).unwrap(),
//        None => {}
//    }
//    let split = string.split(",");
//    for s in split {
//        println!("{}", s)
//    }
//    let vec = split.collect::<Vec<&str>>();
// OR
//    let vec: Vec<&str> = split.collect();
//    println!("{:?}", vec);
    return Ok(resp);
}

pub fn set_post_details(request: &mut Request) -> IronResult<Response>   {
    let mut resp = Response::new();
    let db_pool = &request.get::<persistent::Read<DbPool>>().unwrap();
    let conn = db_pool.get().unwrap();
//    let data = request.get::<Params>();
//    let slug = request.extensions.get::<Router>().unwrap().find("id").unwrap();
//    println!("{:?}", data});
    return Ok(resp);
}