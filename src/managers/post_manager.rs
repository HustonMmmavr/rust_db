use queries::forum as f_q;
use queries::user as u_q;
use queries::thread as t_q;
use models::thread::*;
use models::forum::*;
use models::user::*;
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

use postgres_binary_copy;
use streaming_iterator;
use postgres_binary_copy::BinaryCopyReader;
use streaming_iterator::StreamingIterator;
use postgres::types::{ToSql, INT4, VARCHAR, TIMESTAMPTZ, INT4_ARRAY, TEXT };
use managers::forum_manager as f_m;
use managers::forum_manager::*;
use queries::forum::*;
use queries::post::*;
use managers::user_manager::*;
use managers::thread_manager::get_thread;
use serde_json::from_str;

pub fn create_posts(thread: &Thread, json_posts: Vec<JsonPost>, conn: &PostgresConnection) -> Result<Vec<Post>, i32> {
    let created: chrono::DateTime<Utc> = Utc::now();
    let mut posts: Vec<Post> = Vec::new();

    let mut db_posts: Vec<DbPost> = Vec::new();
    for json_post in json_posts {
        let mut post: Post;
        let forum: String = thread.forum.to_string();
        let u_id;
        let u_name;
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

        let mut f_id:i32 = 0;
        let query = conn.query(GET_FORUM_ID, &[&thread.forum]).unwrap();

        for row in &query {
            f_id = row.get("id");
        }

        let message = json_post.message.unwrap();

        let mut pst = Post{ id: p_id, author: u_name.clone(),
            message: message.clone(), forum: thread.forum.to_string(), thread: thread.id,
            parent: 0, created: format!{"{:?}", created }, isEdited: false
        };

        let mut dbPst = DbPost { id: p_id, author_id: u_id, author_name: u_name.clone(),
            message: message.clone(), forum_id: f_id, forum_slug: thread.forum.to_string(), thread: thread.id,
            parent: 0, created: created
        };

        if json_post.parent == None || json_post.parent == Some(0) {
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

            let parent = json_post.parent.unwrap();
            pst.set_parent(&parent);
            dbPst.set_parent(&parent);

        }

        posts.push(pst);
        db_posts.push(dbPst);
    }

    let types = &[INT4, INT4, INT4, TEXT, INT4, TEXT, TIMESTAMPTZ, TEXT, INT4];
    let mut data: Vec<Box<ToSql>> = vec![];

    for db_post in db_posts {
        data.push(Box::new(db_post.id as i32));
        data.push(Box::new(db_post.parent));
        data.push(Box::new(db_post.author_id));
        data.push(Box::new(db_post.author_name));
        data.push(Box::new(db_post.forum_id));
        data.push(Box::new(db_post.forum_slug));
        data.push(Box::new(db_post.created));
        data.push(Box::new(db_post.message));
        data.push(Box::new(db_post.thread));
    }

    let data = streaming_iterator::convert(data.into_iter()).map_ref(|v| &**v);
    let mut reader = BinaryCopyReader::new(types, data);
    let stmt = conn.prepare(COPY_POSTS).unwrap();
    stmt.copy_in(&[], &mut reader).unwrap();

    return Ok(posts);
}



//public List<PostModel> findSorted(ThreadModel threadModel, Integer limit, Integer since, String sort, Boolean desc) {
//List<Object> list = new ArrayList<>();
//StringBuilder builder = new StringBuilder();
//
//list.add(threadModel.getId());
//if (since != null) {
//list.add(since);
//}
//if (limit != null) {
//list.add(limit);
//}
//
//if (sort == null)
//sort = "flat";
//
//String sortOrder;
//String signSort;
//if (desc == Boolean.TRUE) {
//sortOrder = " DESC ";
//signSort = " < ";
//}
//else {
//sortOrder = " ASC ";
//signSort = " > ";
//}
//
//switch (sort) {
//case "flat" :
//builder.append(QueryForPost.flatOrTreeposts());
//if (since != null) {
//builder.append(" AND id ");
//builder.append(signSort + "? ");
//}
//builder.append(" ORDER BY id ");
//builder.append(sortOrder);
//if (limit != null) {
//builder.append(" LIMIT ?");
//}
//break;
//case "tree" :
//builder.append(QueryForPost.flatOrTreeposts());
//if (since != null) {
//builder.append(" AND path_to_post");
//builder.append(signSort);
//builder.append("(SELECT path_to_post FROM posts WHERE id = ?)");
//}
//builder.append(" ORDER BY path_to_post ");
//builder.append(sortOrder);
//if (limit != null) {
//builder.append(" LIMIT ?");
//}
//break;
//case "parent_tree" :
//builder.append(QueryForPost.findPosts());
//builder.append("WHERE id_of_root IN (SELECT id FROM posts WHERE thread_id = ? AND parent_id = 0 ");
//if (since != null) {
//builder.append(" AND path_to_post");
//builder.append(signSort);
//builder.append("(SELECT path_to_post FROM posts WHERE id = ?) ");
//}
//builder.append(" ORDER BY id ");
//builder.append(sortOrder);
//if (limit != null) {
//builder.append(" LIMIT ?");
//}
//builder.append(")");
//builder.append("ORDER BY path_to_post ");
//builder.append(sortOrder);
//break;
//default:
//break;
//}
//return jdbcTemplate.query(builder.toString(), list.toArray(), _getPostModel);
//}

//
//if since.len() > 0 {
////        query += "AND _user.nickname ";
////        query += if desc == true  {"< "} else {"> "};
////        query += &format!("${}::CITEXT ", counter);
////        counter+=1;
//args.push(Box::new(since));
//}
//
//query += "ORDER BY _user.nickname ";
//query += if desc == true {"DESC "} else {" "};
//let mut lim: i64 = 0;
//if limit > 0 {
//query += &format!("LIMIT ${}", counter);
//lim = limit as i64;
//args.push(Box::new(lim));
//counter += 1;
//}

pub fn get_posts_sort(slug: &str, limit: i32, desc: bool, since: String, sort: String, conn: &PostgresConnection) -> Result<Vec<Post>, i32> {
    use queries::thread::{SEARCH_THREAD, FIND_THREAD_ID_BY_SLUG};

//    let id: i32 = 0;
    let mut t_query;
    match from_str::<i32>(&slug) {
        Ok(val) => {
            t_query = conn.query(SEARCH_THREAD, &[&val]).unwrap();
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
        args.push(Box::new(since.clone()));
    }

    if limit > 0 {
        let lim = limit as i64;
        args.push(Box::new(lim));
    }

    let mut counter = 2;
    if sort == "flat" {
        query += FLAT_OR_THREE_SORT;
        if since.len() > 0 {
            query += &format!("AND id = ${}", counter);
            counter += 1;
            query += sign_sort;
        }

        query += " ORDER BY id ";
        query += sort_order;

        if limit > 0 {
            query += &format!(" LIMIT ${}", counter);
            counter += 1;
        }
    }

    //builder.append(QueryForPost.flatOrTreeposts());
//if (since != null) {
//builder.append(" AND path_to_post");
//builder.append(signSort);
//builder.append("(SELECT path_to_post FROM posts WHERE id = ?)");
//}
//builder.append(" ORDER BY path_to_post ");
//builder.append(sortOrder);
//if (limit != null) {
//builder.append(" LIMIT ?");

    if sort == "tree" {
        query += FLAT_OR_THREE_SORT;
        if since.len() > 0 {
            query += " AND path_to_post ";//&format!("AND path_to_post = ${}", counter);
//            counter += 1;
            query += sign_sort;
            query += &format!("(SELECT path_to_post FROM posts WHERE id = ${})", counter);
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
            query += &format!("SELECT path_to_post FROM posts WHERE id = ${}", counter);
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