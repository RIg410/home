#[macro_use]
extern crate rocket;
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate futures;

use crate::home::Home;
use crate::infra::hap_infra::Accessory;
use crate::infra::telegram::TBot;
use anyhow::Result;
use infra::mtrf_infra::mtrf_wire;
use std::sync::Arc;

mod devices;
mod home;
mod homekit;
mod infra;

#[get("/")]
async fn hello() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() {
    env_logger::init();
    let (_home, hap_devs) = init_state().unwrap();

    let homekit = homekit::init(hap_devs);
    let web_server = rocket::build().mount("/", routes![hello]).launch();

    let (homekit, web_server) = join!(web_server, homekit);
    homekit.unwrap();
    web_server.unwrap();
}

fn init_state() -> Result<(Arc<Home>, Vec<Accessory>)> {
    let bot = TBot::new()?;

    let (home, handlers) = home::init()?;
    let (mtrf, hap) = handlers.into_inner();
    mtrf_wire::init(mtrf, bot, home.clone())?;

    Ok((home, hap))
}
