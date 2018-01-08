
use postgres::rows::Row;
#[macro_use]
use serde_derive;
use serde_json;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Status  {
    pub user: i64,
    pub forum: i64,
    pub thread: i64,
    pub post: i64
}

impl Status {
    pub fn set_user(&mut self, data: i64) {
        self.user = data;
    }

    pub fn set_forum(&mut self, data: i64) {
        self.forum = data;
    }

    pub fn set_thread(&mut self, data: i64) {
        self.thread = data;
    }

    pub fn set_post(&mut self, data: i64) {
        self.post = data;
    }
}

pub  fn empty_status() -> Status {
    return Status{ user: 0, forum: 0, thread: 0, post: 0};
}