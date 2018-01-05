#[macro_use]
use serde_derive;
use serde_json;
use postgres::rows::Row;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JsonUser {
    pub id: Option<i32>,
    pub nickname: Option<String>,
    pub  about: Option<String>,
    pub fullname: Option<String>,
    pub email: Option<String>,
}


#[derive(Serialize, Debug)]
pub struct User {
    pub id: i32,
    pub nickname: String,
    pub about: String,
    pub fullname: String,
    pub email: String,
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
