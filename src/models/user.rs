// Define the `User` model here.
// Include fields, database schema, and related methods.
// Example: Add logic for serialization/deserialization or database queries.

// Path: //rusty-axum/src/modles/user.rs
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Deserialize, Serialize)]
pub struct CreateUserRequest {
    pub user_uuid: Option<String>, // Make this optional
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub password: String,
    pub profile_picture_url: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub gender: Option<String>,
    pub user_type: Option<String>,
    pub is_email_verified: Option<bool>,
    pub is_phone_verified: Option<bool>,
    pub is_admin: Option<bool>,
    pub status: Option<String>,
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub message: String,
    pub user_id: String, // Change this to a String
}