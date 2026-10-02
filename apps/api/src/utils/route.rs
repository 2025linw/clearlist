use axum::http::HeaderMap;
use chrono::Utc;
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;
use subtle::ConstantTimeEq;
use tracing::error;

use crate::error::route::Error;

type HmacSha256 = Hmac<Sha256>;

pub const WEBHOOK_ID: &str = "x-webhook-id";
pub const WEBHOOK_TIMESTAMP: &str = "x-webhook-timestamp";
pub const WEBHOOK_SIGNATURE: &str = "x-webhook-signature";

const MAX_TIMESTAMP_AGE: i64 = 5 * 60;

pub fn verify_webhook_signature(
    secret: &str,
    headers: &HeaderMap,
    body: &[u8],
) -> Result<(), Error> {
    if cfg!(test) {
        return Ok(());
    }

    let webhook_id = headers
        .get(WEBHOOK_ID)
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            error!(target: "clearlist_api", "failed to get webhook id header");

            Error::Unauthenticated
        })?;

    let timestamp = headers
        .get(WEBHOOK_TIMESTAMP)
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            error!("failed to get webhook timestamp header");

            Error::Unauthenticated
        })?;

    let signature = headers
        .get(WEBHOOK_SIGNATURE)
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            error!("failed to get webhook signature header");

            Error::Unauthenticated
        })?;

    let timestamp_value: i64 = timestamp.parse().map_err(|err| {
        error!("timestamp header was not a number: {err}");

        Error::Unauthenticated
    })?;
    let now = Utc::now().timestamp();
    if (now - timestamp_value).abs() > MAX_TIMESTAMP_AGE {
        error!("webhook token exprired");

        return Err(Error::Unauthenticated);
    }

    let expected = create_webhook_bytes(
        secret.as_bytes(),
        webhook_id.as_bytes(),
        timestamp.as_bytes(),
        body,
    );

    let signature = signature.strip_prefix("v1=").ok_or_else(|| {
        error!("signature missing version prefix");

        Error::Unauthenticated
    })?;

    let provided = hex::decode(signature).map_err(|_| {
        error!("unable to decode signature");

        Error::Unauthenticated
    })?;

    if provided.len() != expected.len() || !bool::from(provided.ct_eq(expected.as_slice())) {
        error!("signature did not match");

        return Err(Error::Unauthenticated);
    }

    Ok(())
}

fn create_webhook_bytes(
    secret: &[u8],
    webhook_id: &[u8],
    timestamp: &[u8],
    body: &[u8],
) -> [u8; 32] {
    let mut mac = HmacSha256::new_from_slice(secret).expect("HMAC accepts keys of any length");
    mac.update(webhook_id);
    mac.update(b".");
    mac.update(timestamp);
    mac.update(b".");
    mac.update(body);

    mac.finalize().into_bytes().into()
}

pub fn create_webhook_signature(
    secret: &[u8],
    webhook_id: &[u8],
    timestamp: &[u8],
    body: &[u8],
) -> String {
    hex::encode(create_webhook_bytes(secret, webhook_id, timestamp, body))
}
