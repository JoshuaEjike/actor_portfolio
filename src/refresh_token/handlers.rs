use axum::{Json, extract::State};

use tokio::sync::oneshot;
use tower_cookies::Cookies;

use crate::{
    errors::api_errors::ApiErrors,
    refresh_token::messages::RefreshTokenMessage,
    state::AppState,
    utils::cookies::{clear_refresh_cookies, set_refresh_cookie},
};

pub async fn refresh(
    State(state): State<AppState>,
    cookies: Cookies,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let refresh = cookies
        .get("refresh_token")
        .ok_or(ApiErrors::Unauthorized("Missing refresh token".into()))?
        .value()
        .to_string();

    println!("{refresh:?}");

    let (tx, rx) = oneshot::channel();

    state
        .refresh_token_tx
        .send(RefreshTokenMessage::Refresh {
            refresh_token: refresh,
            respond_to: tx,
        })
        .await
        .map_err(|_| ApiErrors::InternalServerError("Service unavailable".to_string()))?;

    let tokens = rx
        .await
        .map_err(|_| ApiErrors::InternalServerError("Failed".to_string()))??;

    cookies.add(set_refresh_cookie(tokens.refresh_token));

    Ok(Json(
        serde_json::json!({ "access_token": tokens.access_token }),
    ))
}

pub async fn logout(State(state): State<AppState>, cookies: Cookies) -> Result<(), ApiErrors> {
    if let Some(cookie) = cookies.get("refresh_token") {
        let (tx, rx) = oneshot::channel();

        state
            .refresh_token_tx
            .send(RefreshTokenMessage::Logout {
                refresh_token: cookie.value().into(),
                respond_to: tx,
            })
            .await
            .map_err(|_| ApiErrors::InternalServerError("Service unavailable".to_string()))?;

        rx.await
            .map_err(|_| ApiErrors::InternalServerError("Failed".to_string()))??;
    }

    cookies.remove(clear_refresh_cookies());

    Ok(())
}

// pub async fn login(
//     State(state): State<AppState>,
//     Json(payload): Json<LoginRequest>,
//     cookies: Cookies,
// ) -> Result<Json<serde_json::Value>, ApiErrors> {
//     let user = authenticate_user(payload).await?;

//     let (tx, rx) = oneshot::channel();

//     state
//         .refresh_token_tx
//         .send(RefreshTokenMessage::Login {
//             user_id: user.id,
//             respond_to: tx,
//         })
//         .await
//         .map_err(|_| ApiErrors::InternalServerError("Service unavailable".to_string()))?;

//     let tokens = rx
//         .await
//         .map_err(|_| ApiErrors::InternalServerError("Failed".to_string()))??;

//     cookies.add(set_refresh_cookie(tokens.refresh_token));

//     // Ok((
//     //     StatusCode::OK,
//     //     set_refresh_cookie(tokens.refresh_token),
//     //     Json(json!({ "access_token": tokens.access_token })),
//     // ).into_response())
//     Ok(Json(
//         serde_json::json!({ "access_token": tokens.access_token }),
//     ))
// }
