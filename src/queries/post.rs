pub const SELECT_NEXT_POST_ID: &'static str = "SELECT nextval('posts_id_seq')";
pub const SELECT_POST_PATH: &'static str = "SELECT path_to_post FROM posts WHERE id = ?";
pub const SELECT_POST: &'static str = "SELECT created, id, is_edited, message, parent_id, author_id, thread_id, forum_slug, forum_id, author_name FROM posts WHERE ";
//@@get_post_id = @@get_post + "id = ?"
pub const SELELCT_POST_BY_ID: &'static str = "SELECT created, id, is_edited, message, parent_id, author_id, thread_id, forum_slug, forum_id, author_name FROM posts WHERE id = $1";
pub const INSERT_POST: &'static str = "INSERT INTO post(id, parent_id, author_id, created, forum_slug, message, thread_id, id_of_root)\
VALUES($1, $2, $3, $4, $5::CITEXT, $6, $7, $8)";

pub const GET_PARENT_DATA: &'static str = "SELECT thread_id FROM posts WHERE id = $1";

// maybe forum_id
pub const INSERT_POST_BIG: &'static str = "INSERT INTO post(id, parent_id, author_id author_name, forum_slug, created, message, thread_id, id_of_root)\
VALUES($1, $2, $3, $4::CITEXT, $5::CITEXT, $6, $7, $8)";

pub const COPY_POSTS: &'static str = "COPY posts (id, parent_id, author_id, author_name, forum_id,\
 forum_slug, created, message, thread_id) FROM STDIN (FORMAT binary)";

//id SERIAL PRIMARY KEY,
//parent_id    INTEGER     DEFAULT 0,
//author_id   INTEGER REFERENCES userprofiles (id) ON DELETE CASCADE   NOT NULL,
//author_name CITEXT,
//created   TIMESTAMPTZ DEFAULT NOW(),
//forum_id  INTEGER REFERENCES forums (id) ON DELETE CASCADE  NOT NULL,
//forum_slug CITEXT,
//is_edited BOOLEAN     DEFAULT FALSE,
//message   TEXT        DEFAULT NULL,
//thread_id INTEGER REFERENCES threads (id) ON DELETE CASCADE NOT NULL,
//id_of_root INTEGER,
//path_to_post INTEGER []