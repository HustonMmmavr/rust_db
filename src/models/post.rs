use postgres::rows::Row;
use postgres;
use postgres::types::{TIMESTAMPTZ};
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

use models::user::{ User};
use models::forum::{ Forum};
use models::thread::{ Thread};

#[derive(Serialize, Debug, Clone)]
pub struct PostDetails {
    pub author: Option<User>,
    pub forum: Option<Forum>,
    pub thread: Option<Thread>,
    pub post: Option<Post>
}

impl PostDetails {
    pub fn set_user (&mut self, author: User) {
        self.author = Some(author);
    }

    pub fn set_post(& mut self, post: Post) {
        self.post = Some(post);
    }

    pub fn set_thread(&mut self, thread: Thread) {
        self.thread = Some(thread);
    }

    pub fn set_forum(&mut self, forum: Forum) {
        self.forum = Some(forum);
    }
}

pub fn empty_post_details() -> PostDetails {
    return PostDetails{thread: None, author: None, post: None, forum: None};
}

impl Post {
    pub fn set_parent(&mut self, parent: &i32) {
        self.parent = parent.clone();
    }

    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }

    pub fn set_is_edited (&mut self) {
        self.isEdited = true;
    }
}

impl DbPost {
    pub fn set_parent(&mut self, &parent: &i32) {
        self.parent = parent.clone();
    }
}

pub fn read_post(row: &Row) -> Post {
    let data = row.get_bytes("created").unwrap();
    let tz: chrono::DateTime<chrono::Utc> = postgres::types::FromSql::from_sql(&TIMESTAMPTZ, data).unwrap();
    let time = format!("{:?}", tz);
    let id: i32 = row.get("id");
    return Post {
        id: id as i64,
        author: row.get("author_name"),
        message : row.get("message"),
        forum :  row.get("forum_slug"),
        thread  : row.get("thread_id"),
        parent : row.get("parent_id"),
        created: time,
        isEdited : row.get("is_edited")
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