use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Serialize)]
pub struct ResponseMessage<T = String> {
    pub(crate) message: T,
}

#[derive(Serialize)]
pub struct ResponseTokenMessage {
    pub(crate) message: String,
    pub(crate) token: String,
}
