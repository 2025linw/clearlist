use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use axum_extra::extract::CookieJar;
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use reqwest::header::COOKIE;
use serde::Deserialize;
use tracing::error;
use uuid::Uuid;

use crate::{
    GenericAppState,
    category::repo::CategoryRepository,
    error::route::Error::{self, Unauthenticated},
    tag::repo::TagRepository,
    task::repo::TaskRepository,
    user::{repo::UserRepository, types::UserID},
};

#[derive(Debug, Clone, Copy)]
pub struct UserContext {
    pub id: UserID,
    pub tz: Tz,
}

impl<U, T, Ta, C> FromRequestParts<GenericAppState<U, T, Ta, C>> for UserContext
where
    U: UserRepository,
    T: TaskRepository,
    Ta: TagRepository,
    C: CategoryRepository,
{
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &GenericAppState<U, T, Ta, C>,
    ) -> Result<Self, Self::Rejection> {
        match Session::from_request_parts(parts, state).await {
            Ok(session) => {
                let user = state
                    .user_service
                    .get(session.user_id)
                    .await
                    .map_err(|err| {
                        error!("user not found: {err}");
                        Error::InternalServerError("unable to get user data".to_string())
                    })?;

                Ok(Self {
                    id: user.id,
                    tz: user.preferred_timezone.unwrap_or(Tz::UTC),
                })
            }
            Err(err) => Err(err),
        }
    }
}

#[derive(Debug)]
pub struct Session {
    pub user_id: UserID,
    pub name: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub token: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

impl<U, T, Ta, C> FromRequestParts<GenericAppState<U, T, Ta, C>> for Session
where
    U: UserRepository,
    T: TaskRepository,
    Ta: TagRepository,
    C: CategoryRepository,
{
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &GenericAppState<U, T, Ta, C>,
    ) -> Result<Self, Self::Rejection> {
        match IntermediateFormat::from_request_parts(parts, state).await {
            Ok(session) => Ok(Self {
                user_id: UserID(session.user.id),
                name: session.user.name,
                email: session.user.email,
                created_at: session.user.created_at,
                token: session.session.token,
                expires_at: session.session.expires_at,
            }),
            Err(err) => Err(err),
        }
    }
}

pub type OptionalSession = Option<Session>;

impl<U, T, Ta, C> FromRequestParts<GenericAppState<U, T, Ta, C>> for OptionalSession
where
    U: UserRepository,
    T: TaskRepository,
    Ta: TagRepository,
    C: CategoryRepository,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &GenericAppState<U, T, Ta, C>,
    ) -> Result<Self, Self::Rejection> {
        match Session::from_request_parts(parts, state).await {
            Ok(session) => Ok(Some(session)),
            Err(_) => Ok(None),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct IntermediateFormat {
    pub user: IntermediateUser,
    pub session: IntermediateSession,
}

impl<U, T, Ta, C> FromRequestParts<GenericAppState<U, T, Ta, C>> for IntermediateFormat
where
    U: UserRepository,
    T: TaskRepository,
    Ta: TagRepository,
    C: CategoryRepository,
{
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &GenericAppState<U, T, Ta, C>,
    ) -> Result<Self, Self::Rejection> {
        let cookies = CookieJar::from_headers(&parts.headers);
        let session_id = cookies.get(&state.config.cookie_key).ok_or_else(|| {
            error!("cookie key was missing from config");

            Error::Unauthenticated
        })?;

        let auth_req = reqwest::Client::new()
            .get(
                state
                    .config
                    .auth_server_url
                    .clone()
                    .join("/api/auth/get-session")
                    .map_err(|err| {
                        error!("failed to create auth server URL path: {err}");

                        Error::InternalServerError("unable to reach auth server".to_string())
                    })?,
            )
            .header(COOKIE, session_id.to_string());

        let res = auth_req.send().await.map_err(|err| {
            error!("failed to send request to auth server: {err}");

            Error::InternalServerError("unable to get response from auth server".to_string())
        })?;

        if res.status() == StatusCode::UNAUTHORIZED {
            return Err(Unauthenticated);
        } else if !res.status().is_success() {
            error!("request responded with failure");

            return Err(Error::InternalServerError(
                "request on auth server failed".to_string(),
            ));
        }

        let user_session = res.json::<IntermediateFormat>().await.map_err(|err| {
            error!("failed to process response from server: {err}");

            Error::InternalServerError("unable to process response from auth".to_string())
        })?;

        Ok(user_session)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct IntermediateUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    // pub email_verified: bool,
    // pub image: Option<String>,
    pub created_at: DateTime<Utc>,
    // pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct IntermediateSession {
    // pub id: Uuid,
    pub token: String,
    // pub user_id: Uuid,
    // pub user_agent: Option<String>,
    // pub ip_address: Option<String>,
    pub expires_at: DateTime<Utc>,
    // pub created_at: DateTime<Utc>,
    // pub updated_at: DateTime<Utc>,
}
