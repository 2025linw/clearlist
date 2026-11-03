use crate::{
    category::types::route::UpdateRequest,
    error::service::{INVALID_SINGLE_LINE, NO_EMPTY_STRING, TOO_LONG, ValidationError},
    utils::service::is_valid_single_line_string,
};

use super::CreateRequest;

pub fn validate_create_request(request: CreateRequest) -> Result<CreateRequest, ValidationError> {
    let mut request = request;
    normalize_create_request(&mut request);

    if request.name.is_empty() {
        return Err(ValidationError::InvalidValue {
            field: "name",
            reason: NO_EMPTY_STRING,
        });
    } else if !is_valid_single_line_string(&request.name) {
        return Err(ValidationError::InvalidValue {
            field: "name",
            reason: INVALID_SINGLE_LINE,
        });
    } else if request.name.len() > 100 {
        return Err(ValidationError::InvalidValue {
            field: "name",
            reason: TOO_LONG,
        });
    }

    Ok(request)
}

fn normalize_create_request(request: &mut CreateRequest) {
    request.name = request.name.trim().to_owned();
}

pub fn validate_update_request(request: UpdateRequest) -> Result<UpdateRequest, ValidationError> {
    let mut request = request;
    normalize_update_request(&mut request);

    if let Some(name) = &request.name {
        if name.is_empty() {
            return Err(ValidationError::InvalidValue {
                field: "name",
                reason: NO_EMPTY_STRING,
            });
        } else if !is_valid_single_line_string(name) {
            return Err(ValidationError::InvalidValue {
                field: "name",
                reason: INVALID_SINGLE_LINE,
            });
        } else if name.len() > 100 {
            return Err(ValidationError::InvalidValue {
                field: "name",
                reason: TOO_LONG,
            });
        }
    }

    Ok(request)
}

fn normalize_update_request(request: &mut UpdateRequest) {
    if let Some(name) = request.name.as_mut() {
        *name = name.trim().to_owned();
    }
}
