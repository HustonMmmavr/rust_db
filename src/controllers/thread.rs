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
use serde_json::from_str;

use queries::user as u_q;
use models::user as u_model;
use models::error::{ErrorMsg};
use self::u_model::{User, JsonUser, empty_user, copy_user, read_user};
use managers::user_manager as u_manager;
use models::post::*;
use models::thread::*;
use managers::thread_manager::*;
use managers::post_manager as p_m;
use managers::post_manager::*;
use managers::user_manager::*;
use queries::post::*;
use models::vote::*;
use params;



pub fn create_posts(request : &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let db_pool = &request.get::<persistent::Read<DbPool>>().unwrap();
    let conn = db_pool.get().unwrap();
    let raw = request.get::<bodyparser::Raw>().unwrap().unwrap();
    let slug_or_id = request.extensions.get::<Router>().unwrap().find("slug_or_id").unwrap_or("/");

    let mut thread_option;
    match from_str::<i32>(slug_or_id) {
        Ok(val) => thread_option = get_thread(&val, &conn),
        Err(e) => thread_option = get_thread_by_slug(&slug_or_id.to_string(), &conn)
    }

    let mut thread;
    match thread_option {
        Ok(d) => thread = d,
        Err(err) => {
            resp.set_mut(JsonResponse::json(ErrorMsg{message: "Not found"})).set_mut(status::NotFound);
            return Ok(resp);
        }
    }

    let json_posts: Vec<JsonPost> = serde_json::from_str(&raw).unwrap();

    if json_posts.len() == 0 {
        resp.set_mut(JsonResponse::json(json_posts)).set_mut(status::Created);
        return Ok(resp);
    }

//    match p_m::create_posts(&thread, json_posts, &conn) {
        match p_m::create_posts(&thread, json_posts, db_pool) {

        Ok(val) => {
            resp.set_mut(JsonResponse::json(val)).set_mut(status::Created);
            return Ok(resp);
        }
        Err(val) => if (val == 409) {
            resp.set_mut(JsonResponse::json(ErrorMsg{message: "Not found thread"})).set_mut(status::Conflict);
            return Ok(resp);
        } else {
            resp.set_mut(JsonResponse::json(ErrorMsg{message: "Not found user"})).set_mut(status::NotFound);
            return Ok(resp);
        }
    }
    return Ok(resp);
}


pub fn get_thread_(request : &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let db_pool = &request.get::<persistent::Read<DbPool>>().unwrap();
    let conn = db_pool.get().unwrap();
//    let raw = request.get::<bodyparser::Raw>().unwrap().unwrap();
    let slug_or_id = request.extensions.get::<Router>().unwrap().find("slug_or_id").unwrap_or("/");

//    let mut thread_option;2
    match from_str::<i32>(slug_or_id) {
        Ok(val) => {
            match get_thread(&val, &conn) {
                Ok(thread) => {
                    resp.set_mut(JsonResponse::json(thread)).set_mut(status::Ok);
                }
                Err(_) => {
                    resp.set_mut(JsonResponse::json(ErrorMsg{message: "Not found"})).set_mut(status::NotFound);
                }
            }
        }
        Err(_)  => {
            match get_thread_by_slug(&slug_or_id.to_string(), &conn) {
                Ok(thread) => {
                    resp.set_mut(JsonResponse::json(thread)).set_mut(status::Ok);
                }
                Err(_) => {
                    resp.set_mut(JsonResponse::json(ErrorMsg{message: "Not found"})).set_mut(status::NotFound);
                }
            }

        }
    }
    return Ok(resp);
}

pub fn get_posts(request: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    let db_pool = &request.get::<persistent::Read<DbPool>>().unwrap();
    let conn = db_pool.get().unwrap();
    let data = request.get::<Params>();
    let slug = request.extensions.get::<Router>().unwrap().find("slug_or_id").unwrap();

    println!("{:?}", data);
    let map = data.unwrap();
    let mut limit = -1;
    match map.find(&["limit"]) {
        Some(val) =>
            limit =  params::FromValue::from_value(val).unwrap(),

        None => {}
    }

    let mut desc = false;
    match map.find(&["desc"]) {
        Some(val) =>
            desc = params::FromValue::from_value(val).unwrap(),
        None => {}
    }

//    println!("{}",desc);

    let mut since = String::new();
    match map.find(&["since"]) {
        Some(val) => since = params::FromValue::from_value(val).unwrap(),
        None => {}
    }

//    println!("{}", since);

    let mut sort = String::new();
    match map.find(&["sort"]) {
        Some(val) => sort = params::FromValue::from_value(val).unwrap(),
        None => sort = "flat".to_string()
    }

    match get_posts_sort(slug, limit, desc, since, sort, &conn) {
        Ok(val) => {
            resp.set_mut(JsonResponse::json(val)).set_mut(status::Ok);
        }
        Err(_) => {
            resp.set_mut(JsonResponse::json(ErrorMsg{message: "Not found"})).set_mut(status::NotFound);
        }
    }
    return Ok(resp);
}

pub fn update_thread_(request: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let db_pool = &request.get::<persistent::Read<DbPool>>().unwrap();
    let conn = db_pool.get().unwrap();
    let json_thread = request.get::<bodyparser::Struct<JsonThreadUpdate>>().unwrap().unwrap();
    let slug_or_id = request.extensions.get::<Router>().unwrap().find("slug_or_id").unwrap().to_string();

    match update_thread(&slug_or_id, &json_thread, &conn) {
        Ok(val) => {
            resp.set_mut(JsonResponse::json(val)).set_mut(status::Ok);
            Ok(resp)
        }
        Err(e) =>   {
            resp.set_mut(JsonResponse::json(ErrorMsg{message: "Not found"})).set_mut(status::NotFound);
            return Ok(resp);
        }

    }
}

pub fn vote_ (request: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let db_pool = &request.get::<persistent::Read<DbPool>>().unwrap();
    let conn = db_pool.get().unwrap();
    let json_vote = request.get::<bodyparser::Struct<Vote>>().unwrap().unwrap();
    let slug_or_id = request.extensions.get::<Router>().unwrap().find("slug_or_id").unwrap().to_string();

    match vote(json_vote, slug_or_id, &conn) {
        Ok(val) => {
            resp.set_mut(JsonResponse::json(val)).set_mut(status::Ok);
            return Ok(resp);
        }
        Err(e) => {
            resp.set_mut(JsonResponse::json(ErrorMsg{message: "Not found"})).set_mut(status::NotFound);
            return Ok(resp);
        }
    }

    return Ok(resp);
}