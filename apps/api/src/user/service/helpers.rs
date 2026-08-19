use crate::{
    error::service::{NO_EMPTY_STRING, NO_NONSPACE_WHITESPACE, ValidationError},
    utils::service::is_valid_single_line_string,
};

use super::{ProvisionRequest, UpdateRequest};

pub fn validate_provision_request(
    request: ProvisionRequest,
) -> Result<ProvisionRequest, ValidationError> {
    let mut request = request;
    normalize_create_request(&mut request);

    if request.display_name.is_empty() {
        return Err(ValidationError::InvalidValue {
            field: "display_name",
            reason: NO_EMPTY_STRING,
        });
    } else if !is_valid_single_line_string(&request.display_name) {
        return Err(ValidationError::InvalidValue {
            field: "display_name",
            reason: NO_NONSPACE_WHITESPACE,
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
        } else if !is_valid_single_line_string(display_name) {
            return Err(ValidationError::InvalidValue {
                field: "display_name",
                reason: NO_NONSPACE_WHITESPACE,
            });
        }
    }

    Ok(request)
}

fn normalize_create_request(request: &mut ProvisionRequest) {
    request.display_name = request.display_name.trim().to_owned();
}

fn normalize_update_request(request: &mut UpdateRequest) {
    if let Some(display_name) = request.display_name.as_mut() {
        *display_name = display_name.trim().to_owned();
    }
}
