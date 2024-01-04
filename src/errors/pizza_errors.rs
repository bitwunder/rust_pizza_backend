use derive_more::Display;
use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
    body::BoxBody,
};


use PizzaError::*;


#[derive(Display, Debug)]
pub enum PizzaError {
    NoPizzasFound,
    CreationError,
    NoSuchPizza,
}

impl ResponseError for PizzaError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            NoPizzasFound => StatusCode::NOT_FOUND,
            CreationError => StatusCode::INTERNAL_SERVER_ERROR,
            NoSuchPizza => StatusCode::NOT_FOUND,
        }
    }
}