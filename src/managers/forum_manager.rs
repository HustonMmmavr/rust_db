use queries::forum as f_q;
use queries::user as u_q;
use models::forum::*;
use db::{PostgresConnection};
use models::user::{User, read_user};
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
//
//def get_users(params)
//p params
//ds = @db.fetch(@@search_forum, params['slug'])
//p ds
//p ds[:data]
//if ds.count == 0
//return {:data => @@not_found, :status => 'NO_RES'}
//end
//
//f_id = ds.first[:id]
//
//query = @@get_users + "forum_id = :forum_id )"
//since = params['since']
//desc = params['desc']
//limit = params['limit']
//hash = {}
//hash[:forum_id] = f_id
//if since != nil
//query += " AND u.nickname " + (desc == "true" ? " < :since::CITEXT " : " > :since::CITEXT ")
//hash[:since] = since
//end
//
//query += "ORDER BY u.nickname" + (desc == "true" ? " DESC " : " ")
//
//if limit != nil
//query += "LIMIT :limit"
//hash[:limit] = limit
//end
//
//ds = @db.fetch(query, hash)
//arr = ds.all
//# arr.each do |data|
//#   data[:author] = data.delete(:author_name)
//#   data[:forum] = data.delete(:forum_slug)
//# end
//return {:data => arr, :status=> 'OK'}
//end

pub fn get_users(slug: &str, conn: &PostgresConnection) -> Result<Vec<User>, i32> {
    let query = conn.query(f_q::get_forum, &[&slug]).unwrap();
    if (query.len() == 0) {
        return Err(404);
    }

    let mut forum = empty_forum();
//    for row in &query {
//        forum.id = row.get("id");
//        read_forum(&mut forum, row);
//    }

    let users: Vec<User> = Vec::new();
    println!("{:?}", forum);
    return Ok(users);
}



pub fn get_threads() {

}