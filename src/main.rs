#[macro_use]
extern crate derive_builder;

use crate::api::profile::profile::get_profile_by_id;
use actix_web::{App, HttpServer};
use tokio::join;
use api::freecompany::profile::get_free_company_by_id;
use api::linkshell::linkshell::get_linkshell_by_id;

mod api;
mod lodestone;
mod bot;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bot = bot::DiscordBot::new();
    let bot_fut = bot.start();

    let http_fut = HttpServer::new(|| App::new()
        .service(get_profile_by_id)
        .service(get_free_company_by_id)
        .service(get_linkshell_by_id)
    ).bind(("localhost", 9999)).unwrap().run();

    let ret = join!(bot_fut, http_fut);
    ret.1
}
