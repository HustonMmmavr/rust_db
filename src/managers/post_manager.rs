use queries::forum as f_q;
use queries::user as u_q;
use queries::thread as t_q;
use models::thread::*;
use models::forum::*;
use models::user::*;
use postgres::types::ToSql;
use db::{PostgresConnection};
use std;
use postgres;
use postgres::Error;
use postgres::error::SqlState;
use chrono::Utc;
use chrono;
use time;
use chrono::prelude::*;
use time::Duration;
use std::str::FromStr;
use models::post::*;
use managers::user_manager::*;
use queries::post::*;

pub fn create_posts(thread: &Thread, json_posts: Vec<JsonPost>, conn: &PostgresConnection) -> Result<Vec<Post>, i32> {
    let created: chrono::DateTime<Utc> = Utc::now();
    let mut posts: Vec<Post> = Vec::new();

//    let stmt = conn.prepare(INSERT_POST_BIG).unwrap();

    for json_post in json_posts {
        let mut post: Post;
        let forum: String = thread.forum.to_string();
        let u_id;
        let u_name;
        println!("{:?}", json_post);
        match  find_user_id_and_nick(&json_post.author.unwrap(), &conn) {
            Ok(val) => {
                let (name, id) = val;
                u_name = name;
                u_id = id;
            },
            Err(err) => {
                return Err(404);
            }
        }

        let mut p_id: i64 = 0;
        for row in &conn.query(SELECT_NEXT_POST_ID, &[]).unwrap() {
            p_id = row.get(0);
        }

        let post = empty_post();
        if json_post.parent == None || json_post.parent == Some(0) {
            let pst = Post{ id: p_id, author: u_name.to_string(),
            message: json_post.message.unwrap().to_string(), forum: thread.forum.to_string(), thread: thread.id,
            parent: 0, created: format!{"{:?}", created }, isEdited: false
            };
            posts.push(pst);
        } else {
            let mut parent_thread_id: i32 = 0;
            let query = conn.query(GET_PARENT_DATA, &[&json_post.parent]).unwrap();
            if query.len() == 0 {
                return Err(409);
            }
            for row in &query {
                parent_thread_id = row.get(0);
            }

            if parent_thread_id != thread.id {
                return Err(409)
            }
        }
    }

    return Ok(posts);
}
