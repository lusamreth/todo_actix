use actix_web;
mod db;
mod domain;
mod driver;
mod gateway;
mod port;
mod usecase;

#[macro_use]
extern crate dotenv_codegen;

#[actix_rt::main]
async fn main() {
    println!("Hello, world!");
    let gmm = dotenv!("DB_HOST");
    println!("gm {}", gmm);
}

async fn run() {}
