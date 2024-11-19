

// Implement utilities for hashing and verifying passwords.
// Example: Use libraries like `argon2` or `bcrypt` for secure password handling.

// use argon2::{self, Config};
// use rand::Rng;

// // Constants:
// const SALT_LENGTH: usize = 32;  // Salt length for password hashing if same ppassword if two user have

// pub fn hash_password(password: &str) -> Result<String, argon2::Error> {
//     let salt: Vec<u8> = rand::thread_rng().gen_iter().take(SALT_LENGTH).collect();
//     let config = Config::default();
//     argon2::hash_encoded(password.as_bytes(), &salt, &config)
// }

// pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool, argon2::Error> {
//     argon2::verify_encoded(password.as_bytes(), hashed_password).unwrap_or(false)
// }
