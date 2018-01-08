use queries::forum as f_q;
use queries::user as u_q;
use models::forum::*;
use db::{PostgresConnection};
use models::user::{User, read_user};
use queries::forum::*;
use postgres::types::{ToSql};
use models::user::*;
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
            let mut new_forum = get_forum(&forum.slug, conn).unwrap();
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

pub fn get_users(slug: &str, limit: i32, desc: bool, since: String, conn: &PostgresConnection) -> Result<Vec<User>, i32> {
    let forum_query = conn.query(GET_FORUM_ID, &[&slug]).unwrap();
    if forum_query.len() == 0 {
        return Err(404);
    }


    let mut f_id:i32  = 0;
    for row in &forum_query {
        f_id = row.get(0);//"id")
    }
//
    let mut query = GET_USERS.to_string();
    let mut counter: i32 = 2;

    let mut args = Vec::<Box<ToSql>>::new();
    args.push(Box::new(f_id));

    if since.len() > 0 {
        query += "AND _user.nickname ";
        query += if desc == true  {"< "} else {"> "};
        query += &format!("${}::CITEXT ", counter);
        counter+=1;
        args.push(Box::new(since));
    }

    query += "ORDER BY _user.nickname ";
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

    let mut users: Vec<User> = vec![];
    for row in &query_rows {
        let mut user = read_user(&row);
        users.push(user);
    }
    return Ok(users);
}

pub fn count(conn: &PostgresConnection) -> i32 {
    let query = conn.query("SELECT COUNT(*) FROM forums", &[]).unwrap();
    let mut cnt: i32 = 0;
    for row in &query {
        cnt = row.get(0);
    }
    return cnt;
}

pub fn clear(conn: &PostgresConnection) -> i32 {
    let query = conn.query("DELETE FROM forums", &[]).unwrap();
    return 0;
}
