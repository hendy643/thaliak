use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Builder, Serialize, Deserialize, Clone)]
pub struct LodestoneFreeCompany{
    pub id: i32,
}

impl Display for LodestoneFreeCompany {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", serde_json::to_string(self).unwrap())
    }
}