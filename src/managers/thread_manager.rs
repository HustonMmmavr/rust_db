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
            println!("{:?}", e);
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
