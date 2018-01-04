//mod user {
use conf::*;
use iron::prelude::*;
use iron;
use persistent;
use bodyparser;

use persistent::Read;
use iron::status;
use router::Router;

#[macro_use]
use serde_derive;
use serde_json;
//use bodyparser;
use params::{Params, Value};
use std::io::copy;
//
#[derive(Serialize, Deserialize, Clone, Debug)]
struct JsonUser {
    id: Option<i32>,
    nickname: Option<String>,
    about: String,
    fullname: String,
    email: String,
}

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
    user.email = other.email;
    user.about = other.about;
    user.fullname = other.fullname;
}

struct Nil;
//extern crate persistent;

    pub fn create_user(request : &mut Request) -> IronResult<Response> {
        let db_pool = &request.get::<persistent::Read<DbPool>>().unwrap();
        let conn = db_pool.get().unwrap();
//        let nickname;
//
        let mut user = request.get::<bodyparser::Struct<JsonUser>>();
        let mut dbUser = empty_user();
        match  user {
            Ok(Some(user)) => {
                copy_user(& mut dbUser, user)
//                println!("{}", dbUser.about)
            }
            _ => panic!("No body")
//            Err(e) => panic!("{}", e)
        }

//        let mut nickname;
        let ref nickname = request.extensions.get::<Router>().unwrap().find("nickname").unwrap_or("/");
//        dbUser.nickname = String::new(nickname);
        match conn.execute("INSERT INTO userprofiles (about, email, fullname, nickname) VALUES($1, $2::CITEXT, $3, $4::CITEXT)",
            &[&dbUser.about,  &dbUser.email, &dbUser.fullname, &"per.q6H8L8bSrU7uru".to_string()]) {
            Ok(val) => println!("{}", val),
            Err(e) => {
//                let data = conn.execute("SELECT from userprofiles WHERE nickname = $1::CITEXT", &[&"per.".to_string()]);
                for row in &conn.query("SELECT * from userprofiles WHERE nickname = $1::CITEXT", &[&"per.q6H8L8bSrU7uru".to_string()]).unwrap() {
                    let user = User {
                        id: row.get("id"),
                        nickname: row.get("nickname"),
                        fullname: row.get("fullname"),
                        about: row.get("about"),
                        email: row.get("email")
                    };
                    println!("Found person {}", user.email);
                }
                println!("{}", e);
//                println!("{:?}", data);
            }
        }

//        let name;
//        match map.find(&["nickname"]) {
//            Some(&Value::String(ref nick)) =>  name = nick,
//
//            _ => panic!("No")
//        }

//        nickname = name.clone();
        println!("{}", nickname);
//        let mut nickname;

//        match map.find(&["nickname"]) {
//            Some(&Value::String(ref nick)) =>  nickname = nick,
//            _ => panic!("No")
//        }
//        dbUser.nickname = nickname.clone();
//        println!("{}", nickname);
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