use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct ClientMessage {
    pub to: Uuid,
    pub from: Uuid,
    pub content: String,
}
