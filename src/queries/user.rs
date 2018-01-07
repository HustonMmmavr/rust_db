pub const search_conflict: &'static str = "SELECT about, email, fullname, nickname from userprofiles WHERE nickname=$1::CITEXT or email = $2::CITEXT";
pub const  search_user: &'static str = "SELECT about, email, fullname, nickname from userprofiles WHERE nickname=$1::CITEXT";
pub const insert: &'static str = "INSERT INTO userprofiles (about, email, fullname, nickname) VALUES($1, $2::CITEXT, $3, $4::CITEXT)";
pub const get_user_id: &'static str = "SELECT id FROM userprofiles WHERE nickname = $1::CITEXT";
pub const GET_USER_ID_AND_NICK: &'static str = "SELECT id, nickname FROM userprofiles WHERE nickname = $1::CITEXT";