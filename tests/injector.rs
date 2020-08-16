use async_trait::async_trait;
#[async_trait(?Send)]
pub trait Dep {
    async fn find(&self, id: &str) -> String;
}
use std::future::Future;
use std::pin::Pin;
// finder injector!
struct Injector<T: Dep> {
    Gateway: T,
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
#[actix_rt::test]
async fn test_injector() {
    let new_dep = Injector::inject(Gateway {});
    let res = new_dep.execute("finder").await;
    assert_eq!(res, "omomomom")
}
use actix_web::web;
async fn run_hello(appdata: web::Data<Gateway>) -> web::HttpResponse {
    let res = appdata.find("apple").await;
    return web::HttpResponse::Ok().body(res);
}
#[actix_rt::main]
async fn test_actix() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .app_data(Gateway {})
            .route("/run", web::get().to(run_hello))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
