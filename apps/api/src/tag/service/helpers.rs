use crate::error::service::{
    NO_EMPTY_STRING, NO_WHITESPACE, NO_ZERO_LIMIT, NO_ZERO_PAGE, TOO_LONG, ValidationError,
};

use super::{CreateRequest, URLQueryOpts, UpdateRequest};

pub fn validate_query_opts(query: URLQueryOpts) -> Result<URLQueryOpts, ValidationError> {
    let mut query = query;
    normalize_query_opts(&mut query);

    if let Some(page) = query.page
        && page == 0
    {
        return Err(ValidationError::InvalidValue {
            field: "page",
            reason: NO_ZERO_PAGE,
        });
    }
    if let Some(limit) = query.limit
        && limit == 0
    {
        return Err(ValidationError::InvalidValue {
            field: "limit",
            reason: NO_ZERO_LIMIT,
        });
    }

    if let Some(ref category) = query.category {
        if category.is_empty() {
            return Err(ValidationError::InvalidValue {
                field: "category",
                reason: NO_EMPTY_STRING,
            });
        } else if category.chars().any(|c| c.is_whitespace() && c != ' ') {
            return Err(ValidationError::InvalidValue {
                field: "category",
                reason: NO_WHITESPACE,
            });
        }
    }

    Ok(query)
}

pub fn validate_create_request(request: CreateRequest) -> Result<CreateRequest, ValidationError> {
    let mut request = request;
    normalize_create_request(&mut request);

    if request.label.chars().any(|c| c.is_whitespace() && c != ' ') {
        return Err(ValidationError::InvalidValue {
            field: "label",
            reason: NO_WHITESPACE,
        });
    } else if request.label.len() > 100 {
        return Err(ValidationError::InvalidValue {
            field: "label",
            reason: TOO_LONG,
        });
    }

    if let Some(ref category) = request.category {
        if category.is_empty() {
            return Err(ValidationError::InvalidValue {
                field: "category",
                reason: NO_EMPTY_STRING,
            });
        } else if category.chars().any(|c| c.is_whitespace() && c != ' ') {
            return Err(ValidationError::InvalidValue {
                field: "category",
                reason: NO_WHITESPACE,
            });
        } else if category.len() > 100 {
            return Err(ValidationError::InvalidValue {
                field: "category",
                reason: TOO_LONG,
            });
        }
    }

    Ok(request)
}

pub fn validate_update_request(request: UpdateRequest) -> Result<UpdateRequest, ValidationError> {
    let mut request = request;
    normalize_update_request(&mut request);

    if let Some(ref label) = request.label {
        if label.chars().any(|c| c.is_whitespace() && c != ' ') {
            return Err(ValidationError::InvalidValue {
                field: "label",
                reason: NO_WHITESPACE,
            });
        } else if label.len() > 100 {
            return Err(ValidationError::InvalidValue {
                field: "label",
                reason: TOO_LONG,
            });
        }
    }

    if let Some(Some(ref category)) = request.category {
        if category.is_empty() {
            return Err(ValidationError::InvalidValue {
                field: "category",
                reason: NO_EMPTY_STRING,
            });
        } else if category.chars().any(|c| c.is_whitespace() && c != ' ') {
            return Err(ValidationError::InvalidValue {
                field: "category",
                reason: NO_WHITESPACE,
            });
        } else if category.len() > 100 {
            return Err(ValidationError::InvalidValue {
                field: "category",
                reason: TOO_LONG,
            });
        }
    }

    Ok(request)
}

fn normalize_query_opts(query: &mut URLQueryOpts) {
    query.category = query
        .category
        .as_ref()
        .map(|category| category.trim().to_string());
}

fn normalize_create_request(request: &mut CreateRequest) {
    request.label = request.label.trim().to_string();

    request.category = request
        .category
        .as_ref()
        .map(|category| category.trim().to_string());
}

fn normalize_update_request(request: &mut UpdateRequest) {
    request.label = request.label.as_ref().map(|label| label.trim().to_string());

    request.category = request.category.as_ref().map(|update_opt| {
        update_opt
            .as_ref()
            .map(|category| category.trim().to_string())
    })
}
