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

use models::error::{ErrorMsg};
use managers::forum_manager as f_m;
use managers::thread_manager as t_m;
use models::thread::{Thread, JsonThread, empty_thread, copy_json_thread};


pub fn clear(request: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    println!("here");
//    let db_pool = &request.get::<persistent::Read<DbPool>>().unwrap();
//    let conn = db_pool.get().unwrap();
//
//    let mut forum = request.get::<bodyparser::Struct<JsonForum>>();
//    let mut dbForum = empty_forum();
    resp.set_mut(JsonResponse::json(ErrorMsg{message: {"Ok"}})).set_mut(status::Ok);
    return Ok(resp);
}

pub fn count() {

}