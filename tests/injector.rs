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

async fn proxy(p: String) -> String {
    format!("from proxy async param : {}", p)
}
type Exec = Box<dyn FnOnce(String) -> Pin<Box<dyn Future<Output = String> + 'static>>>;

fn generate_proxy() -> Exec {
    let closure: Exec = Box::new(move |a| Box::pin(proxy(a)));
    return closure;
}

#[actix_rt::test]
async fn test_proxy_gen() {
    let gen = generate_proxy();
    let result: String = gen("apple".to_string()).await;
    println!("{}", result);
}
