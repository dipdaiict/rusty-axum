// Define the `User` model here.
// Include fields, database schema, and related methods.
// Example: Add logic for serialization/deserialization or database queries.

// use serde::{Deserialize, Serialize};
// use diesel::Insertable;
// use crate::schema::users;

// #[derive(Debug, Serialize, Deserialize, Insertable)]
// #[diesel(table_name = users)]
// pub struct NewUser {
//     pub user_uuid: String,
//     pub first_name: String,
//     pub last_name: String,
//     pub email: Option<String>,
//     pub phone_number: Option<String>,
//     pub password: String,
//     pub profile_picture_url: Option<String>,
//     pub date_of_birth: Option<chrono::NaiveDate>, // Ensure this matches `Nullable<Date>` in schema.rs
//     pub gender: Option<String>,
//     pub user_type: String,
//     pub is_email_verified: bool,
//     pub is_phone_verified: bool,
//     pub is_admin: bool,
//     pub status: String,
// }