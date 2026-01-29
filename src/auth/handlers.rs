use axum::{
    Json,
    extract::{Path, State},
};

use tokio::sync::oneshot;
use uuid::Uuid;

use crate::{
    auth::{dto::*, messages::AuthMessage},
    errors::api_errors::ApiErrors,
    extractor::auth_extractor::AuthUser,
    fields::{
        email::Email, password::Password, phone_number::PhoneNumber, roles::Roles, text::Text,
    },
    payload_handler::auth_payload_handler::{LoginRequest, RegisterRequest},
    response::general_response::{ResponseMessage, ResponseTokenMessage},
    state::AppState,
};

pub async fn register(
    AuthUser {
        id,
        email,
        name,
        roles,
    }: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let payload_data = payload.validate()?;

    let (tx, rx) = oneshot::channel();

    if payload_data.roles == "root" {
        return Err(ApiErrors::BadRequest(
            "This Admin level cans't be create".to_string(),
        ));
    }

    if roles.as_str() == "normal" {
        return Err(ApiErrors::BadRequest(
            "Becuase of your ADMIN Level you can not create a user.".to_string(),
        ));
    }

    let email_data = Email::new(&payload_data.email)?;

    let password = Password::new(&payload_data.password)?;

    let name_data = Text::new(&payload_data.name)?;

    let roles_data = Roles::new(&payload_data.roles)?;

    let phone_number = payload_data
        .phone_number
        .as_deref()
        .map(PhoneNumber::new)
        .transpose()?;

    let user = RegisteredData {
        email: email_data,
        password,
        name: name_data,
        phone_number,
        roles: roles_data,
        created_by: id,
        created_by_name: name,
        created_by_email: email,
    };

    state
        .auth_tx
        .send(AuthMessage::Register {
            user,
            respond_to: tx,
        })
        .await
        .map_err(|_| ApiErrors::InternalServerError("Auth service unavailable".to_string()))?;

    let user_id = rx
        .await
        .map_err(|_| ApiErrors::InternalServerError("Auth failed".to_string()))??;

    let response = ResponseMessage {
        message: format!("User created: {user_id}"),
    };

    Ok(Json(serde_json::json!(response)))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let (tx, rx) = oneshot::channel();

    let payload_data = payload.validate()?;

    let email = Email::new(&payload_data.email)?;
    let password = Password::new(&payload_data.password)?;

    state
        .auth_tx
        .send(AuthMessage::Login {
            email,
            password,
            respond_to: tx,
        })
        .await
        .map_err(|_| ApiErrors::InternalServerError("Auth service unavailable".to_string()))?;

    let token = rx
        .await
        .map_err(|_| ApiErrors::InternalServerError("Auth failed".to_string()))??;

    let response = ResponseTokenMessage {
        message: "success".to_string(),
        token,
    };

    Ok(Json(serde_json::json!(response)))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let (tx, rx) = oneshot::channel();

    state
        .auth_tx
        .send(AuthMessage::GetUser {
            user_id,
            respond_to: tx,
        })
        .await
        .map_err(|_| ApiErrors::InternalServerError("Service unavailable".to_string()))?;

    let user = rx
        .await
        .map_err(|_| ApiErrors::InternalServerError("Failed".to_string()))??;

    Ok(Json(serde_json::json!(user)))
}

pub async fn get_all_users(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let (tx, rx) = oneshot::channel();

    state
        .auth_tx
        .send(AuthMessage::GetAllUsers { respond_to: tx })
        .await
        .map_err(|_| ApiErrors::InternalServerError("Service unavailable".to_string()))?;

    let users = rx
        .await
        .map_err(|_| ApiErrors::InternalServerError("Failed".to_string()))??;
    Ok(Json(serde_json::json!(users)))
}

pub async fn update_user(
    AuthUser {
        id,
        email,
        name,
        roles,
    }: AuthUser,
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    // TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let (tx, rx) = oneshot::channel();

    if let Some(data) = payload.roles.clone()
        && data == "root"
    {
        return Err(ApiErrors::BadRequest(
            "This Admin level cans't be create".to_string(),
        ));
    }

    if roles.as_str() == "normal" {
        return Err(ApiErrors::BadRequest(
            "Becuase of your ADMIN Level you can not create a user.".to_string(),
        ));
    }

    if id != user_id {
        return Err(ApiErrors::Unauthorized(
            "You are not allowed to use this route".to_string(),
        ));
    }

    let name_data = payload.name.as_deref().map(Text::new).transpose()?;

    let phone_number = payload
        .phone_number
        .as_deref()
        .map(PhoneNumber::new)
        .transpose()?;

    let roles_data = payload.roles.as_deref().map(Roles::new).transpose()?;

    let user = UpdatedData {
        user_id,
        name: name_data,
        phone_number,
        roles: roles_data,
        edited_by: id,
        edited_by_name: name,
        edited_by_email: email,
    };

    state
        .auth_tx
        .send(AuthMessage::UpdateUser {
            user,
            respond_to: tx,
        })
        .await
        .map_err(|_| ApiErrors::InternalServerError("Service unavailable".to_string()))?;

    rx.await
        .map_err(|_| ApiErrors::InternalServerError("Failed".to_string()))??;

    let response = ResponseMessage {
        message: "success".to_string(),
    };

    Ok(Json(serde_json::json!(response)))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let (tx, rx) = oneshot::channel();

    state
        .auth_tx
        .send(AuthMessage::DeleteUser {
            user_id,
            respond_to: tx,
        })
        .await
        .map_err(|_| ApiErrors::InternalServerError("Service unavailable".to_string()))?;

    rx.await
        .map_err(|_| ApiErrors::InternalServerError("Failed".to_string()))??;

    let response = ResponseMessage {
        message: "success".to_string(),
    };

    Ok(Json(serde_json::json!(response)))
}
