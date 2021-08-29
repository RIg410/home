#[macro_use]
extern crate rocket;
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate futures;

use crate::home::Home;
use crate::infra::mtrf_wire;
use anyhow::Result;
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
    let _home = init_state().unwrap();

    let homekit = homekit::init();
    let web_server = rocket::build().mount("/", routes![hello]).launch();

    let (homekit, web_server) = join!(web_server, homekit);
    homekit.unwrap();
    web_server.unwrap();
}

fn init_state() -> Result<Arc<Home>> {
    let (home, handlers) = home::init()?;
    let mtrf = handlers.into_inner();
    mtrf_wire::init(mtrf, home.clone())?;
    Ok(home)
}
