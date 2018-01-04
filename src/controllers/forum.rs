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

use queries::forum as f_q;
use queries::user as u_q;
use models::user::{User, empty_user, copy_user};
use models::error::{ErrorMsg};
use models::forum::*;
use managers::forum_manager::*;
use managers::forum_manager as f_m;


pub fn create(request : &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let db_pool = &request.get::<persistent::Read<DbPool>>().unwrap();
    let conn = db_pool.get().unwrap();

    let mut forum = request.get::<bodyparser::Struct<JsonForum>>();
    let mut dbForum = empty_forum();

    match forum {
        Ok(Some(forum)) => {
            copy_forum(&mut dbForum, forum)
        }
        _ => panic!("No body")
    }

    match create_forum(&dbForum, &conn) {
        Ok(forum) => {
            resp.set_mut(JsonResponse::json(dbForum)).set_mut(status::Created);
            return Ok(resp);
        }
        Err(val) => {
            if val == 404 {
                resp.set_mut(JsonResponse::json(ErrorMsg { message: "No such user" })).set_mut(status::NotFound);
                return Ok(resp);
            }
            else {
                let existing_forum = f_m::get_forum(&dbForum.slug, &conn).unwrap();
                resp.set_mut(JsonResponse::json(existing_forum)).set_mut(status::Conflict);
                return Ok(resp);
            }
        }
    }
}

pub fn get_forum(request : &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let db_pool = &request.get::<persistent::Read<DbPool>>().unwrap();
    let conn = db_pool.get().unwrap();

    let mut forum = request.get::<bodyparser::Struct<JsonForum>>();
    let ref slug = request.extensions.get::<Router>().unwrap().find("slug").unwrap_or("/");

    match f_m::get_forum(slug, &conn) {
        Ok(forum) => {
            resp.set_mut(JsonResponse::json(forum)).set_mut(status::Ok);
            return Ok(resp);
        }
        Err(val) => {
            resp.set_mut(JsonResponse::json(ErrorMsg { message: "No forum" })).set_mut(status::NotFound);
            return Ok(resp);
        }
    }
}

pub fn get_users() {

}