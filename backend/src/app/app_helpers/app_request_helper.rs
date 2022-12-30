use std::collections::HashMap;

use actix_web::dev::ServiceRequest;

/// Returns a HashMap of the query strings as key value
pub(crate) fn split_query_string(req: &ServiceRequest) -> HashMap<String, Option<&str>> {
    let q_string = req.query_string();
    let pieces = q_string.split('&');
    let mut response = HashMap::new();
    for a_piece in pieces {
        let mut an_entry = a_piece.split('=');

        match an_entry.next() {
            Some(name) => {
                response.insert(name.to_owned(), an_entry.next());
            }
            None => (),
        }
    }

    response
}

/// Try getting a value from a header entry
pub(crate) fn pluck_from_header(req: &ServiceRequest, field: &str) -> Option<String> {
    let mut response = None;
    if let Some(token) = req.headers().get(field) {
        if !token.is_empty() {
            response = Some(
                token
                    .to_str()
                    .unwrap_or_default()
                    .replace("Bearer", "")
                    .trim()
                    .to_owned(),
            );
        }
    }

    response
}

/// Tries to get the current user token from the request's query string
/// The value must be assigned to either `_token` or `_t`
pub(crate) fn pluck_token_from_query_string(req: &ServiceRequest) -> Option<String> {
    for (name, value) in split_query_string(req) {
        if &name == "_token" || &name == "_t" {
            return Some(value.unwrap_or_default().to_owned());
        }
    }

    None
}

/// Tries to get the current user role from the request's query string.
/// The value must be assigned to either `_role` or `_r`
pub(crate) fn pluck_role_from_query_string(req: &ServiceRequest) -> Option<String> {
    for (name, value) in split_query_string(req) {
        if &name == "_role" || &name == "_r" {
            return Some(value.unwrap_or_default().to_owned());
        }
    }

    None
}

/// Tries to get the current user token from the request header
/// The token is expected to be a `Bearer` token
pub(crate) fn pluck_token_from_header(req: &ServiceRequest) -> Option<String> {
    let mut response = None;
    if let Some(token) = req.headers().get("authorization") {
        if !token.is_empty() {
            response = Some(
                token
                    .to_str()
                    .unwrap_or_default()
                    .replace("Bearer", "")
                    .trim()
                    .to_owned(),
            );
        }
    }

    response
}

/// Tries to get the current user role from the request header
/// The header key must be `role`
pub(crate) fn pluck_role_from_header(req: &ServiceRequest) -> Option<String> {
    pluck_from_header(req, "role")
}

/// Returns the current user's token and role
/// 
/// The values are returned as a tuple where index 0 is the token
pub(crate) fn pluck_token_and_role(req: &ServiceRequest) -> (Option<String>, Option<String>) {
    let mut response = (pluck_token_from_header(req), pluck_role_from_header(req));

    match response.0 {
        None => {
            response.0 = pluck_token_from_query_string(req);
        }
        _ => (),
    }

    match response.1 {
        None => {
          response.1 = pluck_role_from_query_string(req);
        } 
        _ => ()
    }

    response
}
