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
//use chrono::
//use chrono;
use time::Duration;
use std::str::FromStr;
use models::post::*;

pub fn create_posts(thread: &Thread, posts: &Vec<Post>, conn: &PostgresConnection) -> Result<(), i32> {
//    let created = chrono::DateTime::<Utc>::now().unwrap();
    return Ok(());
}