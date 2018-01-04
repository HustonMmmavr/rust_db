#[macro_use]
use serde_derive;
use serde_json;

#[derive(Serialize)]
struct ErrorMsg {
    message: &'static str,
}