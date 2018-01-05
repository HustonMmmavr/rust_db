pub const search_thread_by_id: &'static str = "SELECT id, author_name, forum_slug, slug, title, message, created, votes FROM threads WHERE id=$1";
pub const search_thread_by_slug: &'static str = "SELECT id, author_name, forum_slug, slug, title, message, created, votes FROM threads WHERE slug=$1::CITEXT";
pub const create_thread: &'static str = "SELECT create_thread($1, $2, $3, $4, $5, $6)";