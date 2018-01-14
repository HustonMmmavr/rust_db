use queries::forum as f_q;
use queries::user as u_q;
use queries::thread as t_q;
use models::thread::*;
use models::forum::*;
use models::user::*;
use db::{PostgresConnection, PostgresPool};
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

use postgres_binary_copy;
use streaming_iterator;
use postgres_binary_copy::BinaryCopyReader;
use streaming_iterator::StreamingIterator;
use postgres::types::{ToSql, INT4, VARCHAR, TIMESTAMPTZ, INT4_ARRAY, TEXT };
use managers::forum_manager as f_m;
use managers::forum_manager::*;
use queries::forum::*;
use queries::user::GET_USER_ID_AND_NICK;
use queries::post::*;
use managers::user_manager::*;
use managers::thread_manager::get_thread;
use serde_json::from_str;


pub fn create_posts(thread: &Thread, json_posts: Vec<JsonPost>, pool: &PostgresPool) -> Result<Vec<Post>, i32> {

    let created: chrono::DateTime<Utc> = Utc::now();
    let mut time = format!("{:?}", created);
    let len = time.len();
    time.truncate(len - 4);
    time.push_str("Z");
    let mut posts: Vec<Post> = Vec::new();

    // Search user by prepared
    let conn = pool.get().unwrap();
    let mut f_id:i32 = 0;
    let query = conn.query(GET_FORUM_ID, &[&thread.forum]).unwrap();

    for row in &query {
        f_id = row.get("id");
    }

    let mut insert_query: String = INSERT_POST_BIG.to_string();
//    let values = "(${}, ${}, ${}, ${}::CITEXT, ${}, ${}::CITEXT, ${}, ${}, ${})";
    let mut args: Vec<Box<ToSql>> = Vec::new();
    let mut i = 0;

    let transaction = conn.transaction().unwrap();
    let user_stmnt = transaction.prepare(GET_USER_ID_AND_NICK).unwrap();
    let parent_stmt = transaction.prepare(GET_PARENT_DATA).unwrap();
//    let insert_post = transaction.prepare(INSERT_POST_BIG).unwrap();
    let next_id = transaction.prepare(SELECT_NEXT_POST_ID).unwrap();
    let mut db_posts: Vec<DbPost> = Vec::new();
//    let
    for json_post in json_posts {
        insert_query += &format!(" (${}, ${}, ${}, ${}::CITEXT, ${}, ${}::CITEXT, ${}, ${}, ${}),", i + 1, i+ 2, i+ 3, i+4, i+5, i+6, i+7, i+8, i+9);
        i += 9;
        let mut post: Post;
        let forum: String = thread.forum.to_string();
        let mut  u_id = 0;
        let mut u_name: String = String::new();
        let u_query = user_stmnt.query(&[&json_post.author]).unwrap();
        if u_query.len() == 0 {
            return Err(404);
        }

        for row in &u_query {
            u_id = row.get(0);
            u_name = row.get(1);
        }

        let mut p_id: i64 = 0;
        for row in &next_id.query(&[]).unwrap() {
            p_id = row.get(0);
        }

        let message = json_post.message.unwrap();

        let mut pst = Post{ id: p_id, author: u_name.clone(),
            message: message.clone(), forum: thread.forum.to_string(), thread: thread.id,
            parent: 0, created:  time.clone() , isEdited: false
        };


//        let mut dbPst = DbPost { id: p_id, author_id: u_id, author_name: u_name.clone(),
//            message: message.clone(), forum_id: f_id, forum_slug: thread.forum.to_string(), thread: thread.id,
//            parent: 0, created: created
//        };

        let mut parent: i32 = 0;
        if json_post.parent == None || json_post.parent == Some(0) {
        } else {
            let mut parent_thread_id: i32 = 0;
            let query = parent_stmt.query(&[&json_post.parent]).unwrap();
            if query.len() == 0 {
                return Err(409);
            }
            for row in &query {
                parent_thread_id = row.get(0);
            }

            if parent_thread_id != thread.id {
                return Err(409)
            }

            parent = json_post.parent.unwrap();
            pst.set_parent(&parent);
//            dbPst.set_parent(&parent);
        }

        args.push(Box::new(p_id as i32));
        args.push(Box::new(parent));
        args.push(Box::new(u_id));
        args.push(Box::new(json_post.author.clone()));
        args.push(Box::new(f_id));
        args.push(Box::new(thread.forum.to_string()));
        args.push(Box::new(created));
        args.push(Box::new(message));
        args.push(Box::new(thread.id));
//
//        insert_post.execute(&[&(dbPst.id as i32), &dbPst.parent, &dbPst.author_id, &dbPst.author_name, &dbPst.forum_id, &dbPst.forum_slug,
//            &dbPst.created, &dbPst.message, &dbPst.thread]).unwrap();

        posts.push(pst);
    }

    insert_query.pop();

//    println!("{}", insert_query);
    let binds_borrowed = args.iter().map(|s| &**s).collect::<Vec<_>>();//args.iter().map(|b| &*b as &ToSql).collect::<Vec<_>>();

    let stmt = transaction.prepare(&insert_query).unwrap();
    stmt.execute(&binds_borrowed);

//    let types = &[INT4, INT4, INT4, TEXT, INT4, TEXT, TIMESTAMPTZ, TEXT, INT4];
//    let mut data: Vec<Box<ToSql>> = vec![];
//
//    for db_post in db_posts {
//        data.push(Box::new(db_post.id as i32));
//        data.push(Box::new(db_post.parent));
//        data.push(Box::new(db_post.author_id));
//        data.push(Box::new(db_post.author_name));
//        data.push(Box::new(db_post.forum_id));
//        data.push(Box::new(db_post.forum_slug));
//        data.push(Box::new(db_post.created));
//        data.push(Box::new(db_post.message));
//        data.push(Box::new(db_post.thread));
//    }
//
//    let data = streaming_iterator::convert(data.into_iter()).map_ref(|v| &**v);
//    let mut reader = BinaryCopyReader::new(types, data);
//    let stmt = transaction.prepare(COPY_POSTS).unwrap();
//    stmt.copy_in(&[], &mut reader).unwrap();

    transaction.commit();
    return Ok(posts);
}

pub fn get_posts_sort(slug: &str, limit: i32, desc: bool, since: String, sort: String, conn: &PostgresConnection) -> Result<Vec<Post>, i32> {
    use queries::thread::{SEARCH_THREAD, FIND_THREAD_ID_BY_SLUG};

    let mut t_query;
    let mut v : i32 = 0;
    match from_str::<i32>(&slug) {
        Ok(val) => {

            t_query = conn.query(t_q::FIND_THREAD_ID, &[&val]).unwrap();
            v = val;
        },
        Err(_) => {
            t_query = conn.query(FIND_THREAD_ID_BY_SLUG, &[&slug]).unwrap();
        }
    }

    if t_query.len() == 0 {
        return Err(404);
    }

    let mut t_id: i32 = 0;
    for row in &t_query {
        t_id = row.get("id");
    }

    let sort_order;
    let sign_sort;
    if desc == true {
        sign_sort = " < ";
        sort_order = " DESC ";
    } else {
        sign_sort = " > ";
        sort_order = " ASC ";
    }

    let mut query = String::new();
    let mut args = Vec::<Box<ToSql>>::new();
    args.push(Box::new(t_id));

    if since.len() > 0 {
        let sinc: i32 = from_str(&since).unwrap();
        args.push(Box::new(sinc));
    }

    if limit > 0 {
        let lim = limit as i64;
        args.push(Box::new(lim));
    }

    let mut counter = 2;
    if sort == "flat" {
        query += FLAT_OR_THREE_SORT;
        if since.len() > 0 {
            query += " AND id ";
            query += sign_sort;
            query += &format!("${}", counter);
            counter += 1;
        }

        query += " ORDER BY id ";
        query += sort_order;

        if limit > 0 {
            query += &format!(" LIMIT ${}", counter);
            counter += 1;
        }
    }

    if sort == "tree" {
        query += FLAT_OR_THREE_SORT;
        if since.len() > 0 {
            query += " AND path_to_post ";//&format!("AND path_to_post = ${}", counter);
            query += sign_sort;
            query += &format!(" (SELECT path_to_post FROM posts WHERE id = ${}) ", counter);
            counter += 1;
        }
        query += " ORDER BY path_to_post ";
        query += sort_order;
        if limit > 0 {
            query += &format!(" LIMIT ${}", counter);
            counter += 1;
        }
    }

    if sort == "parent_tree" {
        query += PARENT_TREE_SORT;
        if since.len() > 0 {
            query += " AND path_to_post ";
            query += sign_sort;
            query += &format!(" (SELECT path_to_post FROM posts WHERE id = ${}) ", counter);
            counter += 1;
        }
        query += " ORDER BY id ";
        query += sort_order;
        if limit > 0 {
            query += &format!(" LIMIT ${} ", counter);
            counter += 1;
        }
        query += ") ";
        query += " ORDER BY path_to_post ";
        query += sort_order;
    }

    let binds_borrowed = args.iter().map(|s| &**s).collect::<Vec<_>>();//args.iter().map(|b| &*b as &ToSql).collect::<Vec<_>>();
    let query_rows = conn.query(&query, &binds_borrowed).unwrap();
    let mut posts: Vec<Post> = Vec::new();
    for row in &query_rows {
        let post = read_post(&row);
        posts.push(post);
    }


    return Ok(posts);
}


pub fn get_post(id: i32, related: String, conn: &PostgresConnection) -> Result<PostDetails, i32> {

    let post_query = conn.query(SELELCT_POST_BY_ID, &[&id]).unwrap();
    if post_query.len() == 0 {
        return Err(404);
    }

    let split = related.split(",");
    let vec: Vec<&str> = split.collect();

    let mut post_details: PostDetails = empty_post_details();
    let mut post: Post = empty_post();
    for row in &post_query {
        post = read_post(&row);
    }



    for arg in &vec {
        if arg == &"user" {
            let nick: &str = &post.author;
            post_details.set_user( get_usr_by_nick(nick, &conn).unwrap());
        }
        if arg == &"forum" {
            let forum: &str = &post.forum;
            post_details.set_forum(get_forum(forum, &conn).unwrap());

        }
        if arg == &"thread" {
            let thread: &i32 = &post.thread;
            post_details.set_thread(get_thread(thread, &conn).unwrap());
        }
    }

    post_details.set_post(post);
    return Ok(post_details);

}

pub fn update_post(id: i32, json_post: &JsonPost, conn: &PostgresConnection) -> Result<Post, i32> {
    let post_query = conn.query(SELELCT_POST_BY_ID, &[&id]).unwrap();
    if post_query.len() == 0 {
        return Err(404);
    }

    let mut post: Post = empty_post();
    for row in &post_query {
        post = read_post(&row);
    }

    match json_post.message {
        Some(ref message) => {
            if &post.message != message {
                conn.execute(UPDATE_POST, &[&message, &id]).unwrap();
                let q = conn.query(SELELCT_POST_BY_ID, &[&id]).unwrap();
                post.set_message(message.to_string());
                post.set_is_edited();
            }
        }
        None => {}
    }
    return Ok(post);
}

pub fn count(conn: &PostgresConnection) -> i32 {
    let query = conn.query("SELECT COUNT(*) FROM posts",  &[]).unwrap();
    let mut cnt: i32 = 0;
    for row in &query {
        cnt = row.get(0);
    }
    return cnt;
}

pub fn clear(conn: &PostgresConnection) -> i32 {
    let query = conn.query("DELETE FROM posts", &[]).unwrap();
    return 0;
}