use crate::error::service::ValidationError;

use super::{ProvisionRequest, UpdateRequest};

pub fn validate_create_request(
    request: ProvisionRequest,
) -> Result<ProvisionRequest, ValidationError> {
    let mut request = request;
    normalize_create_request(&mut request);

    Ok(request)
}

pub fn validate_update_request(request: UpdateRequest) -> Result<UpdateRequest, ValidationError> {
    let mut request = request;
    normalize_update_request(&mut request);

    Ok(request)
}

fn normalize_create_request(_request: &mut ProvisionRequest) {}

fn normalize_update_request(_request: &mut UpdateRequest) {}
