// Define the `User` model here.
// Include fields, database schema, and related methods.
// Example: Add logic for serialization/deserialization or database queries.

use serde::{Deserialize, Serialize};
use chrono::NaiveDate; // Importing NaiveDate from chrono

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub user_uuid: String, // Typically generated at the application level
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>, // Optional to handle users logging in with phone number
    pub phone_number: Option<String>, // Optional to handle users logging in with email
    pub password: String, // Assumes it's already hashed
    pub profile_picture_url: Option<String>,
    pub date_of_birth: Option<NaiveDate>, // Correct usage
    pub gender: Option<String>,
    pub user_type: String,
    pub is_email_verified: bool,
    pub is_phone_verified: bool,
    pub is_admin: bool,
    pub status: String, // Default status can be 'active', 'inactive', etc.
}
