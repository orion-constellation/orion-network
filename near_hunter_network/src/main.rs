use::serde::{Deserialize, Serialize};
use::serde_json::Value;

struct { 
    node_id: String,
    node_address: String,
    node_status: Enum(i32),
    message_cache: serde::Value<Map(String, String)>,
    last_sent: Timestamp
}
