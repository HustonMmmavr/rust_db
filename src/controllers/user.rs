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

#[macro_use]
use serde_derive;
use serde_json;
use params::{Params, Value};
use std::io::copy;
use ijr;
use db;
use ijr::{JsonResponseMiddleware, JsonResponse};
//mod queries {pub mod user;};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct JsonUser {
    id: Option<i32>,
    nickname: Option<String>,
    about: Option<String>,
    fullname: Option<String>,
    email: Option<String>,
}
const search_conflict: &'static str = "SELECT about, email, fullname, nickname from userprofiles WHERE nickname=$1::CITEXT or email = $2::CITEXT";
const search_user: &'static str = "SELECT about, email, fullname, nickname from userprofiles WHERE nickname=$1::CITEXT";

#[derive(Serialize)]
struct ErrorMsg {
    message: &'static str,
}

#[derive(Serialize)]
struct User {
    id: i32,
    nickname: String,
    about: String,
    fullname: String,
    email: String,
}


fn empty_user() -> User {
    return User{id : 0, nickname: String::new(), about: String::new(), fullname: String::new(), email: String::new()};
}

fn copy_user(user : &mut User, other:  JsonUser) {
//    user.nickname = other.nickname;
    user.email = other.email.unwrap();
    user.about = other.about.unwrap();
    user.fullname = other.fullname.unwrap();
}


pub fn create_user(request : &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let db_pool = &request.get::<persistent::Read<DbPool>>().unwrap();
    let conn = db_pool.get().unwrap();

    let mut user = request.get::<bodyparser::Struct<JsonUser>>();
    let mut dbUser = empty_user();
    match  user {
        Ok(Some(user)) => {
            copy_user(& mut dbUser, user)
        }
        _ => panic!("No body")
    }

    let ref nickname = request.extensions.get::<Router>().unwrap().find("nickname").unwrap_or("/");
    match conn.execute("INSERT INTO userprofiles (about, email, fullname, nickname) VALUES($1, $2::CITEXT, $3, $4::CITEXT)",
        &[&dbUser.about,  &dbUser.email, &dbUser.fullname, nickname]) {
        Ok(val) => {
            dbUser.nickname = String::from_str(*nickname).unwrap();
            resp.set_mut(JsonResponse::json(dbUser)).set_mut(status::Created);
        }
        Err(e) => {
            let mut users = Vec::<User>::new();
            for row in &conn.query(&search_conflict, &[nickname, &dbUser.email]).unwrap() {
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
    let stmt = conn.prepare(&search_user).unwrap();
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
    let mut args : Vec<&ToSql> = vec![];//Vec::new();
    let mut query = String::new();
    query.push_str("UPDATE userprofiles set ");
    let mut counter : i32 = 1;
    match  user {
        Ok(Some(user)) => {
//            copy_user(& mut dbUser, user)
            match user.about {
                Some(ref about) => {
                    args.push(about);
                    query.push_str("about = $");
                    query.push_str(&counter.to_string());
                    counter += 1;
                    query.push_str(",")
                }
                None => {}
            }

            match user.fullname {
                Some(ref fullname) => {
                    args.push(String::from_str(fullname).unwrap());
                    query.push_str("fullname = $");
                    query.push_str(&counter.to_string());
                    counter += 1;
                    query.push_str(",")
                }
                None => {}
            }

            match user.email {
                Some(ref email) => {
                    args.push(String::from_str(email).unwrap());
                    query.push_str("email = $");
                    query.push_str(&counter.to_string());
                    counter += 1;
                    query.push_str(",")

                }
                None => {}
            }
        }
        _ => panic!("No body")
    }

    let (q, old) = query.split_at(query.len() - 1);
    q.push_str("WHERE nickname = $");
    q.push_str(counter.to_string());
    q.push_str("::CITXET");

    match conn.execute(&q, args.as_slice()) {
        Ok(val) => {
            let stmt = conn.prepare(&search_user).unwrap();
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
        }
        Err(e) => {
            let stmt = conn.prepare(&search_user).unwrap();
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
                        resp.set_mut(JsonResponse::json(user)).set_mut(status::Conflict);
                    }
                }
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
