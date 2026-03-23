use std::str::FromStr;

use axum::{
    extract::{FromRequestParts, Path},
    http::request::Parts,
};

use crate::errors::api_errors::ApiErrors;

pub struct PathParam<T>(pub T);

impl<S, T> FromRequestParts<S> for PathParam<T>
where
    S: Send + Sync,
    T: FromStr,
    T::Err: std::fmt::Display,
{
    type Rejection = ApiErrors;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Path(value): Path<String> = Path::from_request_parts(parts, state)
            .await
            .map_err(|_| ApiErrors::BadRequest("Invalid path parameter".into()))?;

        let parsed = value
            .parse::<T>()
            .map_err(|e| ApiErrors::BadRequest(format!("Invalid parameter: {}", e)))?;

        Ok(PathParam(parsed))
    }
}

// pub struct PathId(pub Uuid);

// impl<S> FromRequestParts<S> for PathId
// where
//     S: Send + Sync,
// {
//     type Rejection = ApiErrors;

//     async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
//         // Reuse Path extractor
//         let Path(id): Path<String> = Path::from_request_parts(parts, state)
//             .await
//             .map_err(|_| ApiErrors::BadRequest("Invalid ID".into()))?;

//         let uuid =
//             Uuid::parse_str(&id).map_err(|_| ApiErrors::BadRequest("Invalid ID format".into()))?;

//         Ok(PathId(uuid))
//     }
// }

// use axum::{
//     async_trait,
//     extract::{FromRequestParts, Path},
//     http::request::Parts,
// };
// use uuid::Uuid;

// use crate::errors::ApiErrors;

// pub struct Id<T>(pub T);

// #[async_trait]
// impl<S> FromRequestParts<S> for Id<Uuid>
// where
//     S: Send + Sync,
// {
//     type Rejection = ApiErrors;

//     async fn from_request_parts(
//         parts: &mut Parts,
//         state: &S,
//     ) -> Result<Self, Self::Rejection> {
//         let Path(id): Path<String> =
//             Path::from_request_parts(parts, state)
//                 .await
//                 .map_err(|_| ApiErrors::BadRequest("Invalid ID".into()))?;

//         let uuid = Uuid::parse_str(&id)
//             .map_err(|_| ApiErrors::BadRequest("Invalid ID format".into()))?;

//         Ok(Id(uuid))
//     }
// }

// pub async fn register_root_admin(
//     State(state): State<AppState>,
//     Id(school_id): Id<Uuid>,
//     Json(payload): Json<AdminRegisterationRequest>,
// )
