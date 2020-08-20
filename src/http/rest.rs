use actix_web::{web,App,HttpServer, HttpResponse, http::{header, StatusCode}};
use actix_rt::System;
use crate::gateway::{Gateway,make_gateway};
use crate::port::{error::PortException, todo_serv::*};
use crate::usecase::Output;

#[actix_rt::main]
pub async fn actix() -> std::io::Result<()>{
    HttpServer::new(
        || App::new()).bind("127.0.0.1")?.run().await
}

use futures::future::{ready,Ready};
use serde::Serialize;

use actix_web::dev::HttpResponseBuilder;

pub enum ContentState{
    FULL(HttpResponseBuilder),
    EMPTY(HttpResponse)
}

pub fn transform_output<T:Serialize>(output:&Output<T>,content_location:&str) -> ContentState{
    match &output.payload {
        Some(_) => {
            let mut res = HttpResponse::build(StatusCode::OK);
            res.header(header::CONTENT_TYPE, "application/json");
            res.header(header::CONTENT_LOCATION, content_location);
            // mount_head(res);
            return ContentState::FULL(res)
        },
        None => {
            let mut res = HttpResponse::build(StatusCode::NO_CONTENT);
            res.content_length(0);
            res.header(header::CONTENT_TYPE, "application/json");
            res.header(header::CONTENT_LOCATION, content_location);
            return ContentState::EMPTY(res.finish())
        }
    }
}
