use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PassWordEntry {
    pub site: String,
    pub email: String,
    pub username: String,
    pub password: String,
}
