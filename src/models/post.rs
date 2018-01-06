use postgres::rows::Row;
#[macro_use]
use serde_derive;
use serde_json;

//private Integer id;
//private Integer parent;
//private String author;
//private String message;
//private Boolean isEdited;
//private String forum;
//private Integer thread;
//private String created;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JsonPost {
//    pub title: Option<String>,
    pub author: Option<String>,
    pub message: Option<String>,
//    pub forum: String,
//    pub thread: i32,
    pub parent: Option<i32>,
//    pub created: Option<String>
}

impl JsonPost {
    pub fn set_author(&mut self, author: String) {
        self.author = Some(author);
    }
}


#[derive(Serialize, Debug)]
pub struct Post {
    pub title: String,
    pub author: String,
    pub message: String,
    pub forum: String,
    pub thread: i32,
    pub parent: i32,
    pub created: String,
    pub isEdited: bool,
}

use chrono::DateTime;
pub struct DbPost {
    pub title: String,
    pub author: i32,
    pub message: String,
    pub forum: String,
    pub thread: i32,
    pub parent: i32,
    pub created: chrono::DateTime::<Utc>,
}

pub fn empty_post () -> Post {
    return Post{title: String::new(), author: String::new(),
    message : String::new(),
    forum :  String::new(),
    thread  : 0,
    parent : 0,
    created: String::new(),
    isEdited : false}
}