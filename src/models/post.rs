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
    pub title: Option<String>,
    pub author: Option<String>,
    pub message: Option<String>,
//    pub forum: String,
//    pub thread: i32,
    pub parent: Option<i32>,
    pub created: Option<String>
}

//#[derive(Serialize, Deserialize, Debug)]
//pub struct JsonPosts {
//    pub posts: Array<Post>,
//}

#[derive(Serialize, Debug)]
pub struct Post {
    pub title: String,
    pub author: String,
    pub message: String,
    pub forum: String,
    pub thread: i32,
    pub parent: i32,
    pub created: Option<String>,
    pub isEdited: bool,
}

