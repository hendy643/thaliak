extern crate core;

use crate::lodestone::structs::freecompany::{LodestoneFreeCompany};
use crate::lodestone::structs::linkshell::{LodestoneLinkShell};
use crate::lodestone::structs::profile::{LodestoneProfile};

mod internal;
pub(crate) mod structs;
mod tests;

#[derive(Default, Clone, Debug)]
pub struct Lodestone {}

impl Lodestone {
    pub fn new() -> Self {
        Lodestone {}
    }

    pub async fn get_profile(&self, id: u64) -> LodestoneProfile {
        internal::profile::get_profile(id).await
    }

    pub async fn get_linkshell(&self, id: u64) -> LodestoneLinkShell {
        internal::linkshell::get_linkshell(id).await
    }

    pub async fn get_free_company(&self, id: u64) -> LodestoneFreeCompany {
        internal::freecompany::get_free_company(id).await
    }
}
