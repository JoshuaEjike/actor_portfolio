use axum::extract::rejection::JsonRejection;
use axum::{
    Json,
    extract::{FromRequest, Request},
};
use serde::de::DeserializeOwned;

use crate::errors::api_errors::ApiErrors;

pub struct RequiredJson<T>(pub T);

impl<S, T> FromRequest<S> for RequiredJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned,
{
    type Rejection = ApiErrors;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match Json::<T>::from_request(req, state).await {
            // ✅ Valid JSON
            Ok(Json(data)) => Ok(RequiredJson(data)),

            // ❌ Missing body
            Err(JsonRejection::MissingJsonContentType(_)) => {
                Err(ApiErrors::BadRequest("Request body is required".into()))
            }

            // ❌ Invalid JSON structure
            Err(JsonRejection::JsonDataError(err)) => {
                Err(ApiErrors::BadRequest(format!("Invalid JSON data: {}", err)))
            }

            // ❌ Malformed JSON
            Err(JsonRejection::JsonSyntaxError(err)) => {
                Err(ApiErrors::BadRequest(format!("Malformed JSON: {}", err)))
            }

            // ❌ Body read error
            Err(JsonRejection::BytesRejection(_)) => {
                Err(ApiErrors::BadRequest("Failed to read request body".into()))
            }

            // ❌ Fallback
            Err(_) => Err(ApiErrors::BadRequest("Invalid request body".into())),
        }
    }
}

// use axum::{
//     extract::{FromRequest, Request},
//     Json,
// };
// use axum::extract::rejection::JsonRejection;
// use serde::de::DeserializeOwned;

// use crate::errors::api_errors::ApiErrors;

// pub struct JsonBody<T>(pub Option<T>);

// impl<S, T> FromRequest<S> for JsonBody<T>
// where
//     S: Send + Sync,
//     T: DeserializeOwned,
// {
//     type Rejection = ApiErrors;

//     async fn from_request(
//         req: Request,
//         state: &S,
//     ) -> Result<Self, Self::Rejection> {
//         match Json::<T>::from_request(req, state).await {
//             // ✅ Valid JSON
//             Ok(Json(data)) => Ok(JsonBody(Some(data))),

//             // ✅ No content-type OR empty body → treat as None
//             Err(JsonRejection::MissingJsonContentType(_)) => Ok(JsonBody(None)),

//             // ❌ Invalid JSON structure
//             Err(JsonRejection::JsonDataError(err)) => {
//                 Err(ApiErrors::BadRequest(format!(
//                     "Invalid JSON data: {}",
//                     err
//                 )))
//             }

//             // ❌ Syntax error (malformed JSON)
//             Err(JsonRejection::JsonSyntaxError(err)) => {
//                 Err(ApiErrors::BadRequest(format!(
//                     "Malformed JSON: {}",
//                     err
//                 )))
//             }

//             // ❌ Body extraction error
//             Err(JsonRejection::BytesRejection(_)) => {
//                 Err(ApiErrors::BadRequest("Failed to read request body".into()))
//             }

//             // ❌ Fallback
//             Err(_) => Err(ApiErrors::BadRequest("Invalid request body".into())),
//         }
//     }
// }
