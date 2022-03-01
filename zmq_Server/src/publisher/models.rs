use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub topic: String,
    pub content: String,
    pub time_sec: String,
}
