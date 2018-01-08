use queries::forum as f_q;
use queries::user as u_q;
use queries::thread as t_q;
use models::thread::*;
use models::forum::*;
use models::user::*;
use db::{PostgresConnection};


pub fn find_user_id(nick: &String, conn: &PostgresConnection) -> Result<i32, i32> {
    let query = conn.query(u_q::get_user_id, &[nick]).unwrap();
    if (query.len() == 0) {
        return Err(404);
    }

    let mut u_id: i32 =0;
    for row in &query {
        u_id = row.get("id");
    }
    return Ok(u_id);
}

pub fn find_user_id_and_nick(nick: &String, conn: &PostgresConnection) -> Result<(String, i32), i32> {
    let query = conn.query(u_q::GET_USER_ID_AND_NICK, &[nick]).unwrap();
    if (query.len() == 0) {
        return Err(404);
    }

    let mut u_id: i32 =0;
    let mut u_name: String =String::new();
    for row in &query {
        u_id = row.get("id");
        u_name = row.get("nickname");
    }
    return Ok((u_name, u_id));
}

pub fn count(conn: &PostgresConnection) -> i32 {
    let query = conn.query("SELECT COUNT(*) FROM userprofiles", &[]).unwrap();
    let mut cnt: i32 = 0;
    for row in &query {
        cnt = row.get(0);
    }
    return cnt;
}

pub fn clear(conn: &PostgresConnection) -> i32 {
    let query = conn.query("DELETE FROM userprofiles", &[]).unwrap();
    return 0;
}