pub const COUNT_QUERY: &'static str = "SELECT (SELECT COUNT(*) from userprofiles) as user, \
        (SELECT COUNT(*) from forums) as forum, (SELECT COUNT(*) FROM threads) as thread,\
        (SELECT COUNT(*) from posts) as post";

pub const DELETE: &'static str = "DELETE FROM userprofiles";