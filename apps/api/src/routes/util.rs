//! # Route Utility Module
//!
//! This module contains utilities used in routes, such as session extractors and rate limiters

use axum::{
    extract::{
        FromRequest, FromRequestParts, Request,
        rejection::{JsonRejection, PathRejection},
    },
    http::{StatusCode, request::Parts},
};
use axum_extra::extract::CookieJar;
use chrono::{DateTime, Utc};
use governor::{clock::QuantaInstant, middleware::NoOpMiddleware};
use reqwest::header::COOKIE;
use serde::Deserialize;
use serde_qs::axum::QsQueryRejection;
use tower_governor::{
    governor::{GovernorConfig, GovernorConfigBuilder},
    key_extractor::SmartIpKeyExtractor,
};
use uuid::Uuid;

use crate::{AppState, routes::Error};

/// Creates a rate limiter
///
/// # Arguments
///
/// * `num_requests`: request quota
/// * `refresh_rate`: rate in which quotas are replenished in quota
pub fn create_rate_limiter(
    num_requests: u32,
    refresh_rate: u64,
) -> GovernorConfig<SmartIpKeyExtractor, NoOpMiddleware<QuantaInstant>> {
    GovernorConfigBuilder::default()
        .key_extractor(SmartIpKeyExtractor)
        .burst_size(num_requests)
        .per_second(refresh_rate)
        .finish()
        .unwrap()
}

/// Session Wrapper Type
///
/// Used to extract session from authentication server response
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SessionWrapper {
    // user: User, # use this only when needed
    session: Session,
}

/// Session Type
///
/// Used for routes that require authorization
///
/// Implements FromRequestParts to allow for use as extractor in handlers
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    id: Uuid,
    token: String,
    user_id: Uuid,
    user_agent: Option<String>,
    ip_address: Option<String>,
    expires_at: DateTime<Utc>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Session {
    /// Gets user_id from session
    pub fn user_id(&self) -> Uuid {
        self.user_id
    }
}

impl FromRequestParts<AppState> for Session {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let cookies = CookieJar::from_headers(&parts.headers);

        let session_id = cookies
            .get(&state.config.cookie_key)
            .ok_or(Error::NotAuthorized)?;

        let auth_req = reqwest::Client::new()
            .get(
                state
                    .config
                    .auth_server_url
                    .clone()
                    .join("/api/auth/get-session")
                    .expect("This should be a valid URL"),
            )
            .header(COOKIE, session_id.to_string());

        let res = auth_req
            .send()
            .await
            .map_err(|_| Error::InternalServer("Unable to check session".to_string()))?;

        if res.status() == StatusCode::UNAUTHORIZED {
            return Err(Error::NotAuthorized);
        } else if !res.status().is_success() {
            return Err(Error::InternalServer(
                "Unknown response from auth server".to_string(),
            ));
        }

        let session_wrapper = res.json::<SessionWrapper>().await.map_err(|_| {
            Error::InternalServer("Invalid session format received from auth server".to_string())
        })?;

        Ok(session_wrapper.session)
    }
}

/// Optional Session Type
///
/// Used for routes that don't require authorization
///
/// Implements FromRequestParts to allow for use as extractor in handlers
pub type OptionalSession = Option<Session>;

impl FromRequestParts<AppState> for OptionalSession {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        match Session::from_request_parts(parts, state).await {
            Ok(session) => Ok(Some(session)),
            Err(_) => Ok(None),
        }
    }
}

/// Custom Path extractor to customize error
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
            Err(rejection) => Err(Error::InvalidRequest(rejection.body_text())),
        }
    }
}

/// Custom Json extractor to customize error
pub struct Json<T>(pub T);

impl<S, T> FromRequest<S> for Json<T>
where
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => Err(Error::InvalidRequest(rejection.body_text())),
        }
    }
}

/// Custom QsForm (Query) extractor to customize error
pub struct Query<T>(pub T);

impl<S, T> FromRequest<S> for Query<T>
where
    serde_qs::axum::QsForm<T>: FromRequest<S, Rejection = QsQueryRejection>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match serde_qs::axum::QsForm::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => Err(Error::InvalidRequest(rejection.to_string())),
        }
    }
}
