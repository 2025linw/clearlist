#![allow(clippy::disallowed_types)]

use axum::{
    extract::{FromRequestParts, rejection::PathRejection},
    http::request::Parts,
};

use crate::error::route::Error;

pub struct Path<T>(pub T);

impl<S, T> FromRequestParts<S> for Path<T>
where
    axum::extract::Path<T>: FromRequestParts<S, Rejection = PathRejection>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Path::<T>::from_request_parts(parts, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => Err(Error::BadRequest(rejection.body_text())),
        }
    }
}
