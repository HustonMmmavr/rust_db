pub const create_forum: &'static str = "INSERT INTO forums(owner_id, title, slug) VALUES ($1, $2, $3::CITEXT)";

pub const GET_FORUM: &'static str = "SELECT  owner_name, title, slug,  posts, threads FROM FORUMS WHERE slug = $1::CITEXT";

pub const get_full_forum: &'static str =  "SELECT  id, owner_name, title, slug,  posts, threads FROM FORUMS WHERE slug = $1::CITEXT";

pub const GET_FORUM_ID: &'static str = "SELECT id FROM forums WHERE slug = $1::CITEXT";

pub const GET_USERS: &'static str =  "SELECT _user.about, _user.email, _user.fullname, _user.nickname \
 FROM userprofiles _user WHERE _user.id IN ( SELECT user_id FROM forums_and_users WHERE forum_id = $1)";
    //"SELECT DISTINCT _user.id, _user.nickname, _user.about, _user.fullname, _user.email \
    //        FROM forums_and_users forums_and_user JOIN userprofiles _user ON (forums_and_user.user_id = _user.id) \
    //         WHERE forums_and_user.forum_id=$1 ";