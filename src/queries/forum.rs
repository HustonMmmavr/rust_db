pub const create_forum: &'static str = "INSERT INTO forums(owner_id, title, slug) VALUES ($1, $2, $3::CITEXT)";
pub const get_forum: &'static str = "SELECT  owner_name, title, slug,  posts, threads FROM FORUMS WHERE slug = $1::CITEXT";
pub const get_full_forum: &'static str =  "SELECT  id, owner_name, title, slug,  posts, threads FROM FORUMS WHERE slug = $1::CITEXT";