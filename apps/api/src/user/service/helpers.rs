use crate::error::service::{NO_EMPTY_STRING, NO_WHITESPACE, ValidationError};

use super::{CreateRequest, UpdateRequest};

pub fn validate_create_request(request: CreateRequest) -> Result<CreateRequest, ValidationError> {
    let mut request = request;
    normalize_create_request(&mut request);

    if request.display_name.is_empty() {
        return Err(ValidationError::InvalidValue {
            field: "display_name",
            reason: NO_EMPTY_STRING,
        });
    } else if request
        .display_name
        .chars()
        .any(|c| c.is_whitespace() && c != ' ')
    {
        return Err(ValidationError::InvalidValue {
            field: "display_name",
            reason: NO_WHITESPACE,
        });
    }

    Ok(request)
}

pub fn validate_update_request(request: UpdateRequest) -> Result<UpdateRequest, ValidationError> {
    let mut request = request;
    normalize_update_request(&mut request);

    if let Some(ref display_name) = request.display_name {
        if display_name.is_empty() {
            return Err(ValidationError::InvalidValue {
                field: "display_name",
                reason: NO_EMPTY_STRING,
            });
        } else if display_name.chars().any(|c| c.is_whitespace() && c != ' ') {
            return Err(ValidationError::InvalidValue {
                field: "display_name",
                reason: NO_WHITESPACE,
            });
        }
    }

    Ok(request)
}

fn normalize_create_request(request: &mut CreateRequest) {
    request.display_name = request.display_name.trim().to_string();
}

fn normalize_update_request(request: &mut UpdateRequest) {
    request.display_name = request
        .display_name
        .as_ref()
        .map(|name| name.trim().to_string());
}
