use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NewGroupRequest {
    id: String,
    name: String,
    creator_id: String,
}
