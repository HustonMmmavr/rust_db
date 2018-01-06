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