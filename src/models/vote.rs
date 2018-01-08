#[macro_use]
use serde_derive;
use serde_json;
use chrono;
use chrono::Local;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Vote {
    pub nickname: String,
    pub voice: i32
}