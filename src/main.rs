#![feature(entry_insert)]
#[macro_use]
extern crate derive_builder;

use crate::api::profile::profile::get_profile_by_id;
use actix_web::{App, HttpServer};
use api::freecompany::profile::get_free_company_by_id;
use api::linkshell::linkshell::get_linkshell_by_id;

mod api;
mod lodestone;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(get_profile_by_id)
        .service(get_free_company_by_id)
        .service(get_linkshell_by_id)
    ).bind(("localhost", 8080))?
    .run()
    .await
}
