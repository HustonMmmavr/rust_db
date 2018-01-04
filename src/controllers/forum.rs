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

#[derive(Serialize, Deserialize, Clone, Debug)]
struct JsonForum {
    slug: String,
    title: String,
    user: String,
}

//const search_by_mail:

#[derive(Serialize)]
struct ErrorMsg {
    message: &'static str,
}

#[derive(Serialize)]
struct Forum {
    id: i32,
    title: String,
    user: String,
    slug: String,
    threads: i32,
    posts: i32
}


fn empty_forum() -> Forum {
    return Forum{id : 0, title: String::new(), slug: String::new(), user: String::new(), threads:0, posts: 0 };
}

fn copy_forum(forum : &mut Forum, other:  JsonForum) {
//    forum.slug = other.slug.unwrap();
//    forum.title = other.title.unwrap();
//    forum.user = other.user.unwrap();
}



//pub fn create() -> IronResult<Response> {
//    let mut resp = Response::new();
//
//    let db_pool = &request.get::<persistent::Read<DbPool>>().unwrap();
//    let conn = db_pool.get().unwrap();
//
//    let mut forum = request.get::<bodyparser::Struct<JsonForum>>();
//    let mut dbForum = empty_user();
//    match  user {
//        Ok(Some(user)) => {
//            copy_user(& mut dbUser, user)
//        }
//        _ => panic!("No body")
//    }
//
//    let ref nickname = request.extensions.get::<Router>().unwrap().find("nickname").unwrap_or("/");
//    match conn.execute("INSERT INTO userprofiles (about, email, fullname, nickname) VALUES($1, $2::CITEXT, $3, $4::CITEXT)",
//                       &[&dbUser.about,  &dbUser.email, &dbUser.fullname, nickname]) {
//        Ok(val) => {
//            dbUser.nickname = String::from_str(*nickname).unwrap();
//            resp.set_mut(JsonResponse::json(dbUser)).set_mut(status::Created);
//        }
//        Err(e) => {
//            let mut users = Vec::<User>::new();
//            for row in &conn.query(&search_conflict, &[nickname, &dbUser.email]).unwrap() {
//                let user = User {
//                    id: 0,
//                    nickname: row.get("nickname"),
//                    fullname: row.get("fullname"),
//                    about: row.get("about"),
//                    email: row.get("email")
//                };
//                users.push(user);
//            }
//            resp.set_mut(JsonResponse::json(users)).set_mut(status::Conflict);
//        }
//    }
//    return Ok(resp);
//}

//pub fn

pub fn get_users() {

}