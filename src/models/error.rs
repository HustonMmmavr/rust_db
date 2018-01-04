#[macro_use]
use serde_derive;
use serde_json;

#[derive(Serialize)]
pub struct ErrorMsg {
    pub message: &'static str,
}