use postgres::rows::Row;
#[macro_use]
use serde_derive;
use serde_json;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JsonForum {
    pub slug: String,
    pub title: String,
    pub user: String
}

#[derive(Serialize, Debug, Clone)]
pub struct Forum {
    pub id: i32,
    pub title: String,
    pub user: String,
    pub slug: String,
    pub threads: i32,
    pub posts: i32
}


pub fn empty_forum() -> Forum {
    return Forum{id : 0, title: String::new(), slug: String::new(), user: String::new(), threads:0, posts: 0 };
}

pub fn copy_forum(forum : &mut Forum, other:  JsonForum) {
    forum.slug = other.slug;
    forum.title = other.title;
    forum.user = other.user;
}

pub fn read_forum(forum: &mut Forum, row: Row) {
    forum.title = row.get("title");
    forum.slug = row.get("slug");
    forum.posts = row.get("posts");
    forum.user = row.get("owner_name");
    forum.threads = row.get("threads");
}