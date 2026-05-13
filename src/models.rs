use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

// --------------------------------------------------
// Struct representing one password entry
// --------------------------------------------------
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub service: String,

    pub username: String,

    pub password: String,

    pub created_at: DateTime<Local>,
}
