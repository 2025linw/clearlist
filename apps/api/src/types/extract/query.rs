#![allow(clippy::disallowed_types)]

use axum::extract::{FromRequest, Request};
use serde_qs::axum::{QsForm, QsQueryRejection};

use crate::error::route::Error;

pub struct Query<T>(pub T);

impl<S, T> FromRequest<S> for Query<T>
where
    QsForm<T>: FromRequest<S, Rejection = QsQueryRejection>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match QsForm::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => Err(Error::BadRequest(rejection.to_string())),
        }
    }
}
