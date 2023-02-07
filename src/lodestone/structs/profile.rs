use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Builder, Serialize, Deserialize, Clone)]
pub struct LodestoneProfile {
    pub id: i32,
    pub name: String,
    pub nameday: String,
    pub race: String,
    pub clan: String,
    pub gender: String,
    pub title: String,
    pub free_company: String,
    pub grand_company_name: String,
    pub grand_company_rank: String,
    pub bio: Vec<String>,
    pub deity: String,
}

impl Display for LodestoneProfile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", serde_json::to_string(self).unwrap())
    }
}
