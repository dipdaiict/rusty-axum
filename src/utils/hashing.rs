use argon2::{Argon2, PasswordHasher, PasswordVerifier, password_hash::{SaltString, rand_core::OsRng, PasswordHash, Error}};

/// Hashes a password using Argon2.
pub fn hash_password(password: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

/// Verifies a password against a stored hash.
pub fn verify_password(hash: &str, password: &str) -> Result<bool, Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
}