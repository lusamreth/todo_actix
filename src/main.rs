use actix_web::{App,HttpServer,web};
mod db;
use actix_rt;
mod domain;
mod driver;
mod gateway;
mod port;
mod usecase;
mod http;
#[macro_use]
extern crate dotenv_codegen;
use http::handlers::create_task_api;

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| App::new().route("/test/create",web::post().to(create_task_api))).bind("127.0.0.1:8088")?.run().await
}