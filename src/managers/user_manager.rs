//use queries::forum as f_q;
//use queries::user as u_q;
//use queries::thread as t_q;
//use models::thread::*;
//use models::forum::*;
//use models::user::*;
//use db::{PostgresConnection};
//
//
//pub fn create_thread(thread: &mut Thread, conn: &PostgresConnection) -> Result<Thread, i32> {
//    let query = conn.query(u_q::get_user_id, &[&forum.user]).unwrap();
//    if (query.len() == 0) {
//        return Err(404);
//    }
//
//    let mut u_id: i32 =0;
//    for row in &query {
//        u_id = row.get("id");
//    }//                println!("Not found");
//
//
//    match conn.execute(f_q::create_forum, &[&u_id, &forum.title, &forum.slug]) {
//        Ok(val) => {
//            let mut new_forum = get_forum(&forum.slug, conn).unwrap();
//            return Ok(new_forum)
//        },
//        Err(err) => return Err(409)
//    }
//
//}