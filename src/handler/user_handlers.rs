// Path: //rusty-axum/src/handler/user_hanlers.rs

use axum::{
    extract::{Json, State},
    response::{IntoResponse, Response},
    http::StatusCode,
};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::user::CreateUserRequest;
use crate::models::user::CreateUserResponse;
use crate::utils::hashing::hash_password;


pub async fn create_user_handler(
    State(pool): State<PgPool>,
    Json(mut payload): Json<CreateUserRequest>,
) -> Response {
    // Generate a new UUID for the user
    let user_uuid = Uuid::new_v4().to_string();
    payload.user_uuid = Some(user_uuid.clone()); // Set it in the payload

    // Hash the password
    let hashed_password = match hash_password(&payload.password) {
        Ok(hash) => hash,
        Err(err) => {
            eprintln!("Failed to hash password: {}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to hash password".to_string(),
            )
                .into_response();
        }
    };

    payload.password = hashed_password;
    let query = r#"
        INSERT INTO users (
            user_uuid, first_name, last_name, email, phone_number, password,
            profile_picture_url, date_of_birth, gender, user_type,
            is_email_verified, is_phone_verified, is_admin, status
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
    "#;

    let result = sqlx::query(query)
        .bind(payload.user_uuid.as_ref().unwrap())
        .bind(&payload.first_name)
        .bind(&payload.last_name)
        .bind(&payload.email)
        .bind(&payload.phone_number)
        .bind(&payload.password)
        .bind(&payload.profile_picture_url)
        .bind(payload.date_of_birth)
        .bind(&payload.gender)
        .bind(&payload.user_type)
        .bind(payload.is_email_verified.unwrap_or(false))
        .bind(payload.is_phone_verified.unwrap_or(false))
        .bind(payload.is_admin.unwrap_or(false))
        .bind(&payload.status)
        .execute(&pool)
        .await;

 
    match result {
        Ok(_) => {
            let response = CreateUserResponse {
                message: "User created successfully".to_string(),
                user_id: user_uuid, // Return the generated UUID
            };
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(err) => {
            eprintln!("Failed to insert user: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CreateUserResponse {
                    message: "Failed to create user".to_string(),
                    user_id: user_uuid, // Still return the UUID for debugging
                }),
            )
                .into_response()
        }
    }
}