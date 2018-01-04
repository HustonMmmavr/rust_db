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
use self::u_model::{User, JsonUser};
use managers::user_manager as u_manager;


pub fn create_user(request : &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let db_pool = &request.get::<persistent::Read<DbPool>>().unwrap();
    let conn = db_pool.get().unwrap();

    let mut user = request.get::<bodyparser::Struct<u_model::JsonUser>>();
    let mut dbUser = empty_user();
    match  user {
        Ok(Some(user)) => {
            copy_user(& mut dbUser, user)
        }
        _ => panic!("No body")
    }

    let ref nickname = request.extensions.get::<Router>().unwrap().find("nickname").unwrap_or("/");
    match conn.execute(u_q::insert,
        &[&dbUser.about,  &dbUser.email, &dbUser.fullname, nickname]) {
        Ok(val) => {
            dbUser.nickname = String::from_str(*nickname).unwrap();
            resp.set_mut(JsonResponse::json(dbUser)).set_mut(status::Created);
        }
        Err(e) => {
            let mut users = Vec::<User>::new();
            for row in &conn.query(&u_q::search_conflict, &[nickname, &dbUser.email]).unwrap() {
                let user = User {
                    id: 0,
                    nickname: row.get("nickname"),
                    fullname: row.get("fullname"),
                    about: row.get("about"),
                    email: row.get("email")
                };
                users.push(user);
            }
            resp.set_mut(JsonResponse::json(users)).set_mut(status::Conflict);
        }
    }
    return Ok(resp);
}


pub fn get_user(request : &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();


    let db_pool = &request.get::<persistent::Read<DbPool>>().unwrap();
    let conn = db_pool.get().unwrap();

    let ref nickname = request.extensions.get::<Router>().unwrap().find("nickname").unwrap_or("/");
    let stmt = conn.prepare(&u_q::search_user).unwrap();
    let mut query = &stmt.query(&[nickname]).unwrap();
    if (query.len() == 0) {
        let err = ErrorMsg {message: "Not found"};
        resp.set_mut(JsonResponse::json(err )).set_mut(status::NotFound);
    }
    else {
        for row in query {
            let user = User {
                id: 0,
                nickname: row.get("nickname"),
                fullname: row.get("fullname"),
                about: row.get("about"),
                email: row.get("email"),
            };
            resp.set_mut(JsonResponse::json(user)).set_mut(status::Ok);
        }
    }

    return Ok(resp);
}

pub fn update_user(request : &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();


    let db_pool = &request.get::<persistent::Read<DbPool>>().unwrap();
    let conn = db_pool.get().unwrap();

    let mut user = request.get::<bodyparser::Struct<JsonUser>>();
    let ref nickname = request.extensions.get::<Router>().unwrap().find("nickname").unwrap_or("/");

    let mut dbUser = empty_user();
    let mut args : Vec<String> = vec![];//Vec::new();
    let mut query = String::new();
    query.push_str("UPDATE userprofiles set ");
    let mut counter : i32 = 1;

    match  user {
        Ok(Some(user)) => {
            match user.about {
                Some(ref about) => {
                    args.push(about.to_string());
                    query += &format!{"about = ${}::CITEXT,", counter};
                    counter += 1;
                }
                None => {}
            }

            match user.fullname {
                Some(ref fullname) => {
                    args.push(fullname.to_string());
                    query += &format!{"fullname = ${}::CITEXT,", counter};
                    counter += 1;
                }
                None => {}
            }

            match user.email {
                Some(ref email) => {
                    args.push(email.to_string());
                    query += &format!{"email = ${}::CITEXT,", counter};
                    counter += 1;
                }
                None => {}
            }
        }
        _ => panic!("No body")
    }

    if counter > 1 {
        let mut result = query.trim_matches(',').to_string();
        result += &format!(" WHERE nickname = ${}::CITEXT", counter);
        args.push(String::from_str(nickname).unwrap());
        let binds_borrowed = args.iter().map(|b| &*b as &ToSql).collect::<Vec<_>>();

        match conn.execute(&result, &binds_borrowed) {
            Ok(val) => {},
            Err(e) => {
                let err = ErrorMsg { message: "Conflict" };
                resp.set_mut(JsonResponse::json(err)).set_mut(status::Conflict);
                return Ok(resp);
            }
        }


    }

    let stmt = conn.prepare(&u_q::search_user).unwrap();
    let mut query = &stmt.query(&[nickname]).unwrap();
    if (query.len() == 0) {
        let err = ErrorMsg { message: "Not found" };
        resp.set_mut(JsonResponse::json(err)).set_mut(status::NotFound);
    } else {
        for row in query {
            let user = read_user(&row);
            resp.set_mut(JsonResponse::json(user)).set_mut(status::Ok);
        }
    }
    return Ok(resp);
}

pub fn count_user(request : &mut Request) -> IronResult<Response> {
    return Ok(Response::with((iron::status::Ok, "Hello World")));

}

fn clear_user(request : &mut Request) -> IronResult<Response> {
    return Ok(Response::with((iron::status::Ok, "Hello World")));

}

//}

//        let name;
//        match map.find(&["nickname"]) {
//            Some(&Value::String(ref nick)) =>  name = nick,
//
//            _ => panic!("No")
//        }

////        nickname = name.clone();
//println!("{}", nickname);
////        let mut nickname;
//
////        match map.find(&["nickname"]) {
////            Some(&Value::String(ref nick)) =>  nickname = nick,
////            _ => panic!("No")
////        }
////        dbUser.nickname = nickname.clone();
////        println!("{}", nickname);
//return Ok(Response::with((iron::status::Ok, "Hello World")));

//            let j = serde_json::to_string(&users)?.unwrap();
//            println!("{}", j)
//            let k = user_to_json(&mut users);
//            k.
//            println!("{}", j)

//    return Ok(Response::with((iron::status::Ok, "Hello World")));
//            println!("{}", val);
//            let a = String::from_str(*nickname).unwrap();
//            println!("{}", a);
//    &"per.q6H8L8bSrU7uru".to_string()

//fn user_to_json(users:  &mut Vec<User> ) -> Result<String, Error> {
//    // Some data structure.
//
//    // Serialize it to a JSON string.
//    let j = serde_json::to_string(&users)?;
//
//    // Print, write to a file, or send to an HTTP server.
//    println!("{}", j);
//
//    Ok(j)
//}




//                let stmt = conn.prepare(&search_conflict).unwrap();
//                let mut query = &stmt.query(&[nickname]).unwrap();
//                if (query.len() == 0) {
//                } else {
//                    for row in query {
//                        let user = User {
//                            id: 0,
//                            nickname: row.get("nickname"),
//                            fullname: row.get("fullname"),
//                            about: row.get("about"),
//                            email: row.get("email"),
//                        };
//                        resp.set_mut(JsonResponse::json(user)).set_mut(status::Conflict);
//                    }
//                }