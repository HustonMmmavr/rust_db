use postgres::rows::Row;
#[macro_use]
use serde_derive;
use serde_json;
use chrono;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JsonPost {
    pub author: Option<String>,
    pub message: Option<String>,
    pub parent: Option<i32>,
}

impl JsonPost {
    pub fn set_author(&mut self, author: String) {
        self.author = Some(author);
    }
}


#[derive(Serialize, Debug, Clone)]
pub struct Post {
//    pub title: String,
    pub id: i64,
    pub author: String,
    pub message: String,
    pub forum: String,
    pub thread: i32,
    pub parent: i32,
    pub created: String,
    pub isEdited: bool,
}

//imp

//use chrono::DateTime;
pub struct DbPost {
    pub id: i64,
    pub author_id: i32,
    pub author_name: String,
    pub message: String,
    pub forum_slug: String,
    pub forum_id: i32,
    pub thread: i32,
    pub parent: i32,
    pub created: chrono::DateTime<chrono::Utc>,
}

impl Post {
    pub fn set_parent(&mut self, parent: &i32) {
        self.parent = parent.clone();
    }
}

impl DbPost {
    pub fn set_parent(&mut self, &parent: &i32) {
        self.parent = parent.clone();
    }
}

pub fn empty_post () -> Post {
    return Post{ id:0, author: String::new(),
    message : String::new(),
    forum :  String::new(),
    thread  : 0,
    parent : 0,
    created: String::new(),
    isEdited : false}
}