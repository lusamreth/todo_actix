use super::resources::todolist::handlers::{all_lists_api, create_list,list_deletion,remove_task};
use crate::usecase::Output;
use actix_web::{
    guard,
    http::{self, header, StatusCode},
    web, App, HttpResponse, HttpServer,
};

use super::config::*;

#[actix_rt::main]
pub async fn actix() -> std::io::Result<()> {
    HttpServer::new(|| App::new())
        .bind("127.0.0.1")?
        .run()
        .await
}

use serde::Serialize;

use actix_web::dev::HttpResponseBuilder;

pub enum ContentState {
    FULL(HttpResponseBuilder),
    EMPTY(HttpResponse),
}

pub fn transform_output<T: Serialize>(output: &Output<T>, content_location: &str) -> ContentState {
    match &output.payload {
        Some(_) => {
            let mut res = HttpResponse::build(StatusCode::OK);
            res.header(header::CONTENT_TYPE, "application/json");
            res.header(header::CONTENT_LOCATION, content_location);
            // mount_head(res);
            return ContentState::FULL(res);
        }
        None => {
            let mut res = HttpResponse::build(StatusCode::NO_CONTENT);
            res.content_length(0);
            res.header(header::CONTENT_TYPE, "application/json");
            res.header(header::CONTENT_LOCATION, content_location);
            return ContentState::EMPTY(res.finish());
        }
    }
}

use listenfd::ListenFd;

// use actix_web::web::
#[actix_rt::main]
pub(crate) async fn build() -> std::io::Result<()> {
    let addrs = dotenv!("PORT");
    dbg!("Establishing server! {}", addrs);
    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(|| {
        App::new()
            .route(
                "/",
                web::get().to(|| HttpResponse::Ok().body("apple jucie".to_string())),
            )
            .configure(creation_cfg)
            .service(
                web::scope("/tasks")
                    .guard(guard::fn_guard(|head| {
                        let has_ct = head.headers.contains_key(http::header::CONTENT_TYPE);
                        return has_ct;
                    }))
                    .configure(creation_cfg)
                    .configure(patcher_cfg)
                    .configure(deletion_cfg)
                    .configure(retrieval_cfg),
            )
            .default_service(web::route().to(|| HttpResponse::MethodNotAllowed()))
            .service(
                web::scope("/todolist")
                    .route("/", web::get().to(all_lists_api))
                    .route("/", web::post().to(create_list))
                    .route("/{id}", web::delete().to(list_deletion))
                    .route("/", web::delete().to(remove_task)),
            )
    });
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind(dotenv!("PORT"))?
    };

    server.run().await
}

// web::route().guard(guard::fn_guard(|head| {
//     head.method != http::Method::GET
// })).to(|| HttpResponse::MethodNotAllowed())
