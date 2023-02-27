#[macro_use]
extern crate derive_builder;

use crate::api::endpoints::{
    profile::profile::*,
    freecompany::freecompany::*,
    linkshell::linkshell::*
};

use actix_web::{App, HttpServer};
use tokio::join;

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
