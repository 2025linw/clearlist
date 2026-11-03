use serde::Deserialize;

// Response
#[derive(Debug, Deserialize)]
pub struct Response<T> {
    pub status: u16,
    pub data: T,
}

#[derive(Deserialize)]
pub struct ErrorResponse {
    pub status: u16,
    pub message: String,
}
