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
use std::io::copy;
use ijr;
use db;
use ijr::{JsonResponseMiddleware, JsonResponse};

use queries::user as u_q;
use models::user as u_model;
use models::error::{ErrorMsg};
use self::u_model::{User, JsonUser, empty_user, copy_user, read_user};
use managers::user_manager as u_manager;
use models::post::*;
pub fn create_posts(request : &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let db_pool = &request.get::<persistent::Read<DbPool>>().unwrap();
    let conn = db_pool.get().unwrap();
//    println!("{:?}", request.get::<bodyparser::Struct<Raw>>());
    let mut r = request.get::<bodyparser::Raw>().unwrap().unwrap();
    println!("{}",r);

    let val: Vec<Post> = serde_json::from_str(&r).unwrap();

//    println!("{:?}", posts);
//    let mut dbForum = empty_forum();

    return Ok(resp);
}