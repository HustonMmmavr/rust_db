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
use queries::thread::*;
use queries::forum::*;
use serde_json;
use postgres::types::{INT4, TIMESTAMPTZ};

pub struct TimeTZ {
    t: Option<DateTime<Utc>>
}

pub fn empty_time() -> TimeTZ {
    return TimeTZ{t: None};
}

pub fn create_thread(thread: &mut Thread, conn: &PostgresConnection) -> Result<Thread, i32> {
    let mut tz = empty_time();

    match thread.created  {
        Some(ref val) => tz = TimeTZ{t: Some(chrono::DateTime::<Utc>::from_str(val).unwrap())},
        None => tz = TimeTZ{t: None}
    }

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

//fn print_type_of<T>(_: &T) {
//    println!("{}", unsafe { std::intrinsics::type_name::<T>() });
//}

//    let forum_id: INT4 = f_id;

//    let mut args: Vec<&str> = Vec::new();

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
//    let args: Vec<Box> Vec::new();
    let mut counter: i32 = 1;

//    let mut desc = false;
//    match _desc {
//        &Some(val) => desc = serde_json::from_str(&val).unwrap(),
//        &None => {}
//    }

//    let

    let mut args = Vec::<Box<ToSql>>::new();
//    values.push(Box::new(sensor_id));
//    values.push(Box::new(datetime));

//    let f_id: i32 = 0;
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

    query += "ORDER BY created ";
    query += if desc == true {"DESC "} else {" "};
    let mut lim: i64 = 0;
    if limit > 0 {
        query += &format!("LIMIT ${}", counter);
        lim = limit as i64;
        args.push(Box::new(lim));
        counter += 1;
    }

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

    let binds_borrowed = args.iter().map(|s| &**s).collect::<Vec<_>>();//args.iter().map(|b| &*b as &ToSql).collect::<Vec<_>>();
//    println!("{:?}", binds_borrowed);
//    println!("{}", query);
    let query_rows = conn.query(&query, &binds_borrowed).unwrap();
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
    let mut threads: Vec<Thread> = Vec::new();
    for row in &query_rows {
        let mut thread: Thread = empty_thread();
        read_thread(&mut thread, row);
        threads.push(thread);
    }
    return Ok(threads);
}


//    let mut created: chrono::DateTime<Utc>;
//    match since {
//        Some(val) => //created =
//    }
//    match