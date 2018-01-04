use queries::forum as f_q;
use queries::user as u_q;
use models::forum::*;
use db::{PostgresConnection};

pub fn create_forum(forum: & Forum, conn: &PostgresConnection) -> Result<Forum, i32>  {
    let query = conn.query(u_q::get_user_id, &[&forum.user]).unwrap();
    if (query.len() == 0) {
        return Err(404);
    }

    let mut u_id: i32 =0;
    for row in &query {
        u_id = row.get("id");
    }

    match conn.execute(f_q::create_forum, &[&u_id, &forum.title, &forum.slug]) {
        Ok(val) => {
            let mut new_forum = empty_forum();
            new_forum.title = forum.title.to_string();
            new_forum.slug = forum.slug.to_string();
            new_forum.user = forum.user.to_string();
            return Ok(new_forum)
        },
        Err(err) => return Err(409)
    }
}

pub fn get_forum(slug: &str, conn: &PostgresConnection)  -> Result<Forum, i32> {
    let query = conn.query(f_q::get_forum, &[&slug]).unwrap();
    if (query.len() == 0) {
        return Err(404);
    }

    let mut forum = empty_forum();
    for row in &query {
        read_forum(&mut forum, row);
    }

    return Ok(forum);

}