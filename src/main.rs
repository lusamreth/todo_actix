mod db;
use actix_rt;
mod domain;
mod driver;
mod gateway;
mod http;
mod port;
mod usecase;
#[macro_use]
extern crate dotenv_codegen;
use http::rest;

use actix_rt::System;
use std::env;
fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_BACKTRACE", "1");

    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "info");
    }

    // env_logger::init();

    let sys = System::new("actix-web-sample");
    if let Err(e) = rest::build() {
        eprintln!("ERROR: {:?}!", e);
    }

    sys.run()
}
