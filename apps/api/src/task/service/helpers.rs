use crate::{
    error::service::{
        NO_NONMULTILINE_WHITESPACE, NO_NONSPACE_WHITESPACE, NO_ZERO_LIMIT, NO_ZERO_PAGE,
        ValidationError,
    },
    tag::types::TagID,
    utils::service::{dedupe_array, is_valid_multiline_string, is_valid_single_line_string},
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

    Ok(query)
}

pub fn validate_create_request(request: CreateRequest) -> Result<CreateRequest, ValidationError> {
    let mut request = request;
    normalize_create_request(&mut request);

    if !is_valid_single_line_string(&request.title) {
        return Err(ValidationError::InvalidValue {
            field: "title",
            reason: NO_NONSPACE_WHITESPACE,
        });
    }

    if let Some(ref notes) = request.notes
        && !is_valid_multiline_string(notes)
    {
        return Err(ValidationError::InvalidValue {
            field: "notes",
            reason: NO_NONMULTILINE_WHITESPACE,
        });
    }

    Ok(request)
}

pub fn validate_update_request(request: UpdateRequest) -> Result<UpdateRequest, ValidationError> {
    let mut request = request;
    normalize_update_request(&mut request);

    if let Some(ref title) = request.title
        && !is_valid_single_line_string(title)
    {
        return Err(ValidationError::InvalidValue {
            field: "title",
            reason: NO_NONSPACE_WHITESPACE,
        });
    }

    if let Some(Some(ref notes)) = request.notes
        && !is_valid_multiline_string(notes)
    {
        return Err(ValidationError::InvalidValue {
            field: "notes",
            reason: NO_NONMULTILINE_WHITESPACE,
        });
    }

    Ok(request)
}

pub fn validate_set_tags(tag_ids: Vec<TagID>) -> Result<Vec<TagID>, ValidationError> {
    let mut tag_ids = tag_ids;
    normalize_set_tags(&mut tag_ids);

    Ok(tag_ids)
}

fn normalize_query_opts(query: &mut URLQueryOpts) {
    if let Some(tags) = query.tags.as_mut() {
        *tags = dedupe_array(tags.to_vec());
    }
}

fn normalize_create_request(request: &mut CreateRequest) {
    request.title = request.title.trim().to_owned();

    if let Some(notes) = request.notes.as_mut() {
        *notes = notes.trim().to_owned();
    }

    request.tags = dedupe_array(request.tags.to_vec());
}

fn normalize_update_request(request: &mut UpdateRequest) {
    request.title = request.title.as_mut().map(|title| title.trim().to_owned());

    if let Some(notes) = request.notes.as_mut().and_then(Option::as_mut) {
        *notes = notes.trim().to_owned();
    }

    if let Some(tags) = request.tags.as_mut() {
        *tags = dedupe_array(tags.to_vec());
    }
}

fn normalize_set_tags(tag_ids: &mut Vec<TagID>) {
    *tag_ids = dedupe_array(tag_ids.to_vec());
}
