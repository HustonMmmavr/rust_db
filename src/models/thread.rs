use postgres::rows::Row;
use postgres::types::TIMESTAMPTZ;
use postgres::types::FromSql;
// use time;
use postgres;
#[macro_use]
use serde_derive;
use serde_json;
use chrono;
use chrono::Local;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JsonThread {
    pub author: String,
    pub created: Option<String>,
    pub message: Option<String>,
    pub title: Option<String>,
    pub slug: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JsonThreadUpdate {
    pub message: Option<String>,
    pub title: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Thread {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub forum: String,
    pub slug: Option<String>,
    pub created: Option<String>,
    pub message: String,
    pub votes: i32
}


pub fn empty_thread() -> Thread {
    return Thread{id : 0, title: String::new(), slug: Some(String::new()), author: String::new(), forum: String::new(),
        created: Some(String::new()), message: String::new(), votes: 0 };
}

pub fn copy_json_thread(thread : &mut Thread, other:  JsonThread) {
    thread.author = other.author;
    thread.created = other.created;
    thread.message = other.message.unwrap();
    thread.title = other.title.unwrap();
    thread.slug = other.slug;
}

pub fn read_thread(thread: &mut Thread, row: Row) {
    thread.title = row.get("title");
    thread.slug = row.get("slug");
    thread.votes = row.get("votes");
    thread.author = row.get("author_name");
    thread.forum = row.get("forum_slug");
    thread.message = row.get("message");
    thread.id = row.get("id");
    let data = row.get_bytes("created").unwrap();
    let tz: chrono::DateTime<chrono::Utc> = postgres::types::FromSql::from_sql(&TIMESTAMPTZ, data).unwrap();
    let time = format!("{:?}", tz);
    thread.created = Some(time.to_string());
}
