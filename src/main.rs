use actix_web;
mod db;
mod domain;
mod driver;
mod gateway;
mod port;
mod usecase;

#[macro_use]
extern crate dotenv_codegen;

use async_trait::async_trait;
#[async_trait(?Send)]
pub trait Dep {
    async fn find(&self, id: &str) -> String;
}
use std::future::Future;
use std::pin::Pin;
// finder injector!
struct Injector<T: Dep> {
    pub Gateway: T,
}

impl<T: Dep> Injector<T> {
    pub fn inject(param: T) -> Self {
        Injector { Gateway: param }
    }
    pub async fn execute(&self, id: &str) -> String {
        self.Gateway.find(id).await
    }
}
pub struct Gateway {}
#[async_trait(?Send)]
impl Dep for Gateway {
    async fn find(&self, _id: &str) -> String {
        return String::from("omomomom");
    }
}
use actix_web::web;
async fn usecase(uge: &impl Dep) -> String {
    uge.find("ssad").await
}
async fn run_hello(appdata: web::Data<Injector<Gateway>>) -> web::HttpResponse {
    // let res = appdata.find("apple").await;
    let res_from_uc = usecase(&appdata.Gateway).await;
    return web::HttpResponse::Ok().body(res_from_uc);
}
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .data(Injector::inject(Gateway {}))
            .route("/run", web::get().to(run_hello))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
