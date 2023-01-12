use std::fmt;

use actix_web::{
    body::BoxBody,
    http::header::{self, TryIntoHeaderValue, HeaderValue},
    web,
    HttpResponse, ResponseError, HttpResponseBuilder,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotAuthenticatedError {
    success: bool,
    code: String,
    message: String,
}

impl Default for NotAuthenticatedError {
    fn default() -> Self {
        Self {
            success: false,
            code: "permission_denied".to_owned(),
            message: "permission denied".to_owned(),
        }
    }
}

// @todo Change the content type of the error to "application/json"
impl ResponseError for NotAuthenticatedError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::UNAUTHORIZED
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let mut response  = HttpResponse::with_body(self.status_code(), serde_json::to_string(&self).unwrap());
        response.headers_mut().insert(header::CONTENT_TYPE, HeaderValue::from_static("application/json"));
        response.map_into_boxed_body()
    }
}

impl fmt::Display for NotAuthenticatedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = serde_json::to_string(&self)
            .unwrap_or_else(|_| String::from("user is not authenticated"));
        write!(f, "{message}")
    }
}
