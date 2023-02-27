use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Builder, Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct LodestoneLinkShell {
    pub id: u64,
    pub name: String,
    pub members: HashMap<u64, String>,
}

impl Display for LodestoneLinkShell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl Responder for LodestoneLinkShell {
    type Body = BoxBody;

    //noinspection DuplicatedCode
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let res_body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(res_body)
    }
}