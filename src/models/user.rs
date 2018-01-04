#[macro_use]
use serde_derive;
use serde_json;
use postgres::rows::Row;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JsonUser {
    id: Option<i32>,
    nickname: Option<String>,
    about: Option<String>,
    fullname: Option<String>,
    email: Option<String>,
}


#[derive(Serialize)]
pub struct User {
    id: i32,
    nickname: String,
    about: String,
    fullname: String,
    email: String,
}

pub fn read_user(row: &Row) -> User {
    return User{ id: 0, nickname: row.get("nickname"), about: row.get("about"), fullname: row.get("fullname"),
        email: row.get("email")};
}



pub fn empty_user() -> User {
    return User{id : 0, nickname: String::new(), about: String::new(), fullname: String::new(), email: String::new()};
}

pub fn copy_user(user : &mut User, other:  JsonUser) {
//    user.nickname = other.nickname;
    user.email = other.email.unwrap();
    user.about = other.about.unwrap();
    user.fullname = other.fullname.unwrap();
}
