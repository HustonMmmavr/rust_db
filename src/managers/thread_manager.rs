use queries::forum as f_q;
use queries::user as u_q;
use queries::thread as t_q;
use models::thread::*;
use models::forum::*;
use models::user::*;
use postgres::types::ToSql;
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
use queries::thread::*;
use queries::forum::*;
use serde_json;
use postgres::types::{INT4, TIMESTAMPTZ};
use serde_json::from_str;
use models::vote::*;
use queries::user::*;

pub struct TimeTZ {
    t: Option<DateTime<Utc>>
}

pub fn empty_time() -> TimeTZ {
    return TimeTZ{t: None};
}

pub fn create_thread(thread: &mut Thread, conn: &PostgresConnection) -> Result<Thread, i32> {
//pub fn create_thread(thread: &mut Thread, pool: &PostgresPool) -> Result<Thread, i32> {
    let mut tz = empty_time();


    match thread.created  {
        Some(ref val) => tz = TimeTZ{t: Some(chrono::DateTime::<Utc>::from_str(val).unwrap())},
        None => tz = TimeTZ{t: None}
    }

//    match conn.query(t_q::create_thread, &[&thread.author, &tz.t, &thread.forum, &thread.message, &thread.slug, &thread.title]) {
    match conn.query(t_q::create_thread, &[&thread.author, &tz.t, &thread.forum, &thread.message, &thread.slug, &thread.title]) {
        Ok(val) => {
            let mut id: i32 = 0;
            for row in &val {
                id = row.get(0);
            }
//            println

            let thread = get_thread(&id, conn).unwrap();
            return Ok(thread);
        }
        Err(e) => {
//            println!("{:?}", e);
            let code = e.code().unwrap().code();
            if code == "23502" {
                return Err(404);
            }
            return Err(409);
        }
    }
}

pub fn create_thread_pool(thread: &mut Thread, pool: &PostgresPool) -> Result<Thread, i32> {
    let mut tz = empty_time();


    match thread.created  {
        Some(ref val) => tz = TimeTZ{t: Some(chrono::DateTime::<Utc>::from_str(val).unwrap())},
        None => tz = TimeTZ{t: None}
    }

//    match conn.query(t_q::create_thread, &[&thread.author, &tz.t, &thread.forum, &thread.message, &thread.slug, &thread.title]) {
    match pool.get().unwrap().query(t_q::create_thread, &[&thread.author, &tz.t, &thread.forum, &thread.message, &thread.slug, &thread.title]) {
        Ok(val) => {
            let mut id: i32 = 0;
            for row in &val {
                id = row.get(0);
            }
//            println

            let thread = get_thread_pool(&id, pool).unwrap();
            return Ok(thread);
        }
        Err(e) => {
//            println!("{:?}", e);
            let code = e.code().unwrap().code();
            if code == "23502" {
                return Err(404);
            }
            return Err(409);
        }
    }
}

pub fn get_thread(id: &i32, conn: &PostgresConnection) -> Result<Thread, i32> {
    let query = conn.query(t_q::search_thread_by_id, &[id]).unwrap();
    if (query.len() == 0) {
        return Err(404);
    }

    let mut thread = empty_thread();
    for row in &query {
        read_thread(&mut thread, row);
    }
    return Ok(thread);
}

pub fn get_thread_pool(id: &i32, pool: &PostgresPool) -> Result<Thread, i32> {
    let query = pool.get().unwrap().query(t_q::search_thread_by_id, &[id]).unwrap();
    if (query.len() == 0) {
        return Err(404);
    }

    let mut thread = empty_thread();
    for row in &query {
        read_thread(&mut thread, row);
    }
    return Ok(thread);
}

pub fn get_thread_by_slug_pool(slug: &String, pool: &PostgresPool ) -> Result<Thread, i32> {
    let query = pool.get().unwrap().query(t_q::search_thread_by_slug, &[&slug]).unwrap();
    if (query.len() == 0) {
        return Err(404);
    }

    let mut thread = empty_thread();
    for row in &query {
        read_thread(&mut thread, row);
    }
    return Ok(thread);
}

pub fn get_thread_by_slug(slug: &String, conn: &PostgresConnection ) -> Result<Thread, i32> {
    let query = conn.query(t_q::search_thread_by_slug, &[&slug]).unwrap();
    if (query.len() == 0) {
        return Err(404);
    }

    let mut thread = empty_thread();
    for row in &query {
        read_thread(&mut thread, row);
    }
    return Ok(thread);
}

pub fn get_threads(slug: &str, limit: i32, desc: bool, since: String,
    conn: &PostgresConnection) -> Result<Vec<Thread>, i32> {
    let forum_query = conn.query(GET_FORUM_ID, &[&slug]).unwrap();
    if forum_query.len() == 0 {
        return Err(404);
    }

    let mut f_id:i32  = 0;
    for row in &forum_query {
        f_id = row.get(0);//"id")
    }

    let mut query = String::new();
    let mut counter: i32 = 1;

    let mut args = Vec::<Box<ToSql>>::new();
    query.push_str(SEARCH_THREAD);
    query += &format!(" WHERE forum_id = ${} ", counter);
    counter+=1;
    args.push(Box::new(f_id));
    let mut created: chrono::DateTime<Utc>;
    if since.len() > 0 {
        query += "AND created ";
        query += if desc == true  {"<= "} else {">= "};
        query += &format!("${}::TIMESTAMPTZ ", counter);
        counter+=1;
//        } else {"=> ?::TIMESTAMPTZ "};
        created = chrono::DateTime::<Utc>::from_str(&since).unwrap();
        args.push(Box::new(created));
    }

    query += "ORDER BY created ";
    query += if desc == true {"DESC "} else {" "};
    let mut lim: i64 = 0;
    if limit > 0 {
        query += &format!("LIMIT ${}", counter);
        lim = limit as i64;
        args.push(Box::new(lim));
        counter += 1;
    }


    let binds_borrowed = args.iter().map(|s| &**s).collect::<Vec<_>>();//args.iter().map(|b| &*b as &ToSql).collect::<Vec<_>>();
    let query_rows = conn.query(&query, &binds_borrowed).unwrap();
    let mut threads: Vec<Thread> = Vec::new();
    for row in &query_rows {
        let mut thread: Thread = empty_thread();
        read_thread(&mut thread, row);
        threads.push(thread);
    }
    return Ok(threads);
}

pub fn update_thread(slug: &String, json_thread: &JsonThreadUpdate, conn: &PostgresConnection) -> Result<Thread, i32> {
    let mut query = "UPDATE threads SET ".to_string();
    let mut args: Vec<Box<ToSql>> = vec![];
    let mut counter: i32 = 1;

    let message;
    if json_thread.message != None {
        let msg = json_thread.message.clone();
        message = msg.unwrap();
        query += &format!("message = ${},", counter);
        args.push(Box::new(message));
        counter += 1;
    }

    let title;
    if json_thread.title != None {
        let tit = json_thread.title.clone();
        title = tit.unwrap();
        query += &format!("title = ${},", counter);
        args.push(Box::new(title));
        counter += 1;
    }

    let mut id: i32 = 0;
    match from_str::<i32>(slug) {
        Ok(val) => {
            id = val;
        }
        Err(e) => {
            id = -1;
        }
    }

    if counter > 1 {
        let mut result = query.trim_matches(',').to_string();
        if id > 0 {
            args.push(Box::new(id));
            result += &format!(" WHERE id = ${} ", counter);
        } else {
            args.push(Box::new(slug.to_string()));
            result += &format!(" WHERE slug = ${}::CITEXT ", counter);
        }

        let binds_borrowed = args.iter().map(|s| &**s).collect::<Vec<_>>();
        let data = conn.execute(&result, &binds_borrowed).unwrap();
    }

    let thread_query;
    if id != -1 {
        thread_query = conn.query(search_thread_by_id, &[&id]).unwrap();
    } else {
        thread_query = conn.query(search_thread_by_slug, &[&slug]).unwrap();
    }

    if thread_query.len() == 0 {
        return Err(404);
    }

    let mut thread: Thread = empty_thread();

    for row in &thread_query {
        read_thread(&mut thread, row);
    }
    return Ok(thread);
}


pub fn vote(vote_mod: Vote, slug: String, conn: &PostgresConnection) -> Result<Thread, i32> {
    let u_id_query = conn.query(get_user_id, &[&vote_mod.nickname]).unwrap();
//    println!("{:?}", u_id_query);
    if u_id_query.len() == 0 {
        return Err(404);
    }

    let mut u_id = 0;
    for row in &u_id_query {
        u_id = row.get(0);
    }

    let mut id: i32 = 0;
    let t_id_query;
    match from_str::<i32>(&slug) {
        Ok(val) => {
            t_id_query = conn.query(FIND_THREAD_ID, &[&val]).unwrap();
//            id = val;.
        }
        Err(e) => {
            t_id_query = conn.query(FIND_THREAD_ID_BY_SLUG, &[&slug]).unwrap();
//            id = -1;
        }
    }
//    println!("{:?}", t_id_query);

    if t_id_query.len() == 0 {
        return Err(404);
    }

    let mut t_id: i32 = 0;
    for row in &t_id_query {
        t_id = row.get(0);
    }



    match conn.execute(CREATE_OR_UPDATE_VOTE, &[&u_id, &t_id, &vote_mod.voice]) {
        Ok(val) => {
            let thread_quer = conn.query(search_thread_by_id, &[&t_id]).unwrap();
            let mut thread: Thread = empty_thread();
            for row in &thread_quer {
                read_thread(&mut thread, row);
            }
            return Ok(thread);
        }
        Err(_) => return Err(404)
    }
    return Err(404);
}


pub fn count(conn: &PostgresConnection) -> i32 {
    let query = conn.query("SELECT COUNT(*) FROM threads", &[]).unwrap();
    let mut cnt: i32 = 0;
    for row in &query {
        cnt = row.get(0);
    }
    return cnt;
}

pub fn clear(conn: &PostgresConnection) -> i32 {
    let query = conn.query("DELETE FROM threads", &[]).unwrap();
    return 0;
}

//fn print_type_of<T>(_: &T) {
//    println!("{}", unsafe { std::intrinsics::type_name::<T>() });
//}

//    let forum_id: INT4 = f_id;

//    let mut args: Vec<&str> = Vec::new();

//    let mut desc = false;
//    match _desc {
//        &Some(val) => desc = serde_json::from_str(&val).unwrap(),
//        &None => {}
//    }

//    let

//    values.push(Box::new(sensor_id));
//    values.push(Box::new(datetime));

//    let f_id: i32 = 0;

//    match since {
//        &Some(val) => {
//            query += "AND created ";
//            query += if desc == true  {"<= "} else {">= "};
//            query += &format!("${}::TIMESTAMPTZ ", counter);
//            counter+=1;
////        } else {"=> ?::TIMESTAMPTZ "};
//            created = chrono::DateTime::<Utc>::from_str(&val).unwrap();
//            args.push(Box::new(created));
//        }
//        &None => {}
//    }

//    let mut created: chrono::DateTime<Utc>;
//    match since {
//        Some(val) => //created =
//    }
//    match

//    let binds_borrowed = args.iter().map(|b| &*b as &ToSql).collect::<Vec<_>>();
//    println!("{}", query);
//    match conn.query(&query, &[&2000i32, &4i64]) {
//        Ok(_) => {}
//        Err(e) => println!("{:?}", e)
//    }
//    if query_rows.len() == 0 {
//        return Err(404);
//    }
//

//    println!("{:?}", binds_borrowed);
//    println!("{}", query);

//    let mut lim: i32 = 0;
//    query += "ORDER BY created ";
//    query += if desc == true {"DESC "} else {" "};
//    match limit {
//        &Some(val) => {
//            query += &format!("LIMIT ${}", counter);
//            lim = serde_json::from_str(&val).unwrap();
//            args.push(Box::new(lim));
//            counter += 1;
//        },
//        &None => {}
//    }

//    println!("{}", query);
//    for arg in &args {
//        print_type_of(arg);
//    }