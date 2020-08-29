use crate::{port::error::*};
use actix_web::http::header;
use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};

type Uri = String;
type PartialUri = String;
#[derive(Serialize, Deserialize)]
pub struct ErrorResponse<T: Serialize> {
    error_type: Uri,
    sub_type: PartialUri,
    detials: String,
    pub instance: PartialUri,
    #[serde(flatten)]
    pub extensions: Option<T>,
}

impl<T: Serialize> ErrorResponse<T> {
    fn new() -> Self {
        ErrorResponse {
            error_type: "http://about:blank".to_string(),
            sub_type: "about:blank".to_string(),
            detials: String::from("No detial avialable!"),
            instance: "N/A".to_string(),
            extensions: None,
        }
    }

    fn assert_detail(&mut self, msg: String) {
        if msg.len() == 0 {
            self.detials = String::from("No detial avialable!")
        }
        self.detials = msg;
    }

    fn assert_instance(&mut self, instance: PartialUri) {
        let partial_uri_format =
            regex::Regex::new(r"^(/)([a-z]+)+(\-?)+(\w++?)").expect("bad regex!");
        if partial_uri_format.is_match(&instance) == false {
            self.instance = String::from("<Invalid instance uri>!")
        }
        if instance.len() == 0 {
            self.instance = String::from("N/A")
        }
        // assert!(intance.len() >= 0)s;
        self.instance = instance;
    }

    fn assert_type(&mut self, mt: &str, st: &str) {
        let partial_uri_format =
            regex::Regex::new(r"^(/)([a-z]+)+(\-?)+(\w++?)").expect("bad regex!");
        if partial_uri_format.is_match(&mt) == false || partial_uri_format.is_match(st) == false {
            self.error_type = format!("http://{}", mt);
            self.sub_type = format!("/{}/{}", mt, st);
        } else {
            self.error_type = mt.to_string();
            self.sub_type = st.to_string();
        }
    }
}

impl ResponseError for PortException {
    fn status_code(&self) -> actix_web::http::StatusCode {
        let status = match &self.interface_type {
            Some(interface) => {
                if let PortError::Internal(_) = interface {
                    StatusCode::INTERNAL_SERVER_ERROR
                } else {
                    dbg!(self.main_type.to_lowercase());
                    match self.main_type.to_lowercase().clone().as_str() {
                        "bussinesserror" => StatusCode::BAD_REQUEST,
                        "operational" => match self.sub_type.to_lowercase().as_str() {
                            "not-found" => StatusCode::NOT_FOUND,
                            "io" | "io-error" => StatusCode::BAD_REQUEST,
                            _ => panic!("Type not implemented!"),
                        },
                        _ => panic!("Type not implemented!"),
                    }
                }
            }
            None => StatusCode::SERVICE_UNAVAILABLE,
        };

        return status;
    }
    fn error_response(&self) -> actix_web::HttpResponse {
        let mut response = HttpResponse::build(self.status_code());
        response.header(header::CONTENT_TYPE, "application/json");
        // response.header("content-encoding", "br");
        response.set(header::Date(SystemTime::now().into()));

        let mut json_res = ErrorResponse::<String>::new();
        json_res.assert_detail(self.message.clone());
        json_res.assert_type(self.main_type.as_str(), self.sub_type.as_str());
        response.json(json_res)
    }
    fn __private_get_type_id__(&self) -> std::any::TypeId
    where
        Self: 'static,
    {
        std::any::TypeId::of::<Self>()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Invalidbundle<T: Serialize> {
    invalids: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fielderror {
    pub(crate) field: String,
    pub(crate) detials: String,
}

/*
Output :
jsonerror {
    code:10,
    ........
    invalids:[{
        name:"completed_time",
        details:"Wrong time formated"
    }]
}
*/
use std::time::SystemTime;
pub trait InvalidParameter {
    fn invalid_input(&self, field_errors: Vec<Fielderror>) -> HttpResponse;
}

impl InvalidParameter for PortException {
    fn invalid_input(&self, field_errors: Vec<Fielderror>) -> HttpResponse {
        let bundled = Invalidbundle {
            invalids: field_errors,
        };
        let mut res = ErrorResponse::new();
        res.error_type = String::from("Input");
        res.sub_type = String::from("Bad parameter");
        res.extensions = Some(bundled);

        let json = header::ContentType::json();
        let mut http_res = HttpResponse::build(StatusCode::BAD_REQUEST);
        http_res.header(header::CONTENT_TYPE, json);

        http_res.set(header::Date(SystemTime::now().into()));
        http_res.json(res)
    }
}

impl<T: Serialize> Into<ErrorResponse<T>> for PortException {
    fn into(self) -> ErrorResponse<T> {
        let mut new_res = ErrorResponse::new();
        new_res.error_type = self.main_type;
        new_res.sub_type = self.sub_type;
        new_res.detials = self.message;
        new_res.extensions = None;
        return new_res;
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BundleError<T: Serialize> {
    pub(crate) message: String,
    pub(crate) errors: Vec<T>,
}
