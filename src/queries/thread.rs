pub const search_thread_by_id: &'static str = "SELECT id, author_name, forum_slug, slug, title, message, created, votes FROM threads WHERE id=$1";
pub const search_thread_by_slug: &'static str = "SELECT id, author_name, forum_slug, slug, title, message, created, votes FROM threads WHERE slug=$1::CITEXT";
pub const create_thread: &'static str = "SELECT create_thread($1, $2::TIMESTAMPTZ, $3, $4, $5, $6)";
pub const SEARCH_THREAD: &'static str = "SELECT id, author_name, forum_slug, slug, title, message, created, votes FROM threads ";
pub const CREATE_OR_UPDATE_VOTE: &'static str = "SELECT create_or_update_vote($1, $2, $3)";
pub const FIND_THREAD_ID: &'static str = "SELECT id from threads where id = $1";
pub const FIND_THREAD_ID_BY_SLUG: &'static str = "SELECT id from threads where slug = $1";