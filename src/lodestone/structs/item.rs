use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Builder, Serialize, Deserialize, Clone, ToSchema)]
struct LodestoneItem {
    pub id: i32,
}

impl Display for LodestoneItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", serde_json::to_string(self).unwrap())
    }
}