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
// use db;
use ijr::{ JsonResponse};

use models::error::{ErrorMsg};
use managers::forum_manager as f_m;
use managers::thread_manager as t_m;
use models::thread::{Thread, JsonThread, empty_thread, copy_json_thread};
use queries::service::*;
use models::status::*;

pub fn clear(request: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    let db_pool = &request.get::<persistent::Read<DbPool>>().unwrap();
    let conn = db_pool.get().unwrap();
    conn.execute(DELETE, &[]).unwrap();
    resp.set_mut(JsonResponse::json(ErrorMsg{message: {"Ok"}})).set_mut(status::Ok);
    return Ok(resp);
}

pub fn status(request: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    let db_pool = &request.get::<persistent::Read<DbPool>>().unwrap();
    let conn = db_pool.get().unwrap();
    let mut status: Status = empty_status();
    let query = conn.query(COUNT_QUERY, &[]).unwrap();
    for row in &query {
        status.set_user(row.get("user"));
        status.set_forum(row.get("forum"));
        status.set_thread(row.get("thread"));
        status.set_post(row.get("post"));
    }
    resp.set_mut(JsonResponse::json(status)).set_mut(status::Ok);
    return Ok(resp);
}