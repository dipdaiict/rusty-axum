use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, DecodingKey, EncodingKey, TokenData};
use serde::{Serialize, Deserialize};
use std::env;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn create_jwt(username: &str) -> String {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let jwt_algorithm = env::var("JWT_ALGORITHM").expect("JWT_ALGORITHM must be set");
    let algorithm = match jwt_algorithm.as_str() {
        "HS256" => Algorithm::HS256,
        "HS384" => Algorithm::HS384,
        "HS512" => Algorithm::HS512,
        _ => panic!("Unsupported algorithm!"),
    };
    let header = Header::new(algorithm);
    let exp_duration_seconds: i64 = env::var("JWT_EXPIRATION_SECONDS")
        .unwrap_or_else(|_| "3600".to_string()) // Default to 1 hour if not set
        .parse()
        .expect("JWT_EXPIRATION_SECONDS must be a valid integer.");
    let exp = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(exp_duration_seconds))
        .expect("Valid timestamp")
        .timestamp() as usize;
    let claims = Claims {
        sub: username.to_string(),
        exp,
    };
    encode(&header, &claims, &EncodingKey::from_secret(jwt_secret.as_bytes()))
        .expect("Token encoding failed.")
}

// Verify the JWT token
pub fn verify_jwt(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let jwt_algorithm = env::var("JWT_ALGORITHM").unwrap_or_else(|_| "HS256".to_string());
    let algorithm = match jwt_algorithm.as_str() {
        "HS256" => Algorithm::HS256,
        "HS384" => Algorithm::HS384,
        "HS512" => Algorithm::HS512,
        _ => panic!("Unsupported algorithm"),
    };
    let validation = Validation::new(algorithm);
    decode::<Claims>(token, &DecodingKey::from_secret(jwt_secret.as_bytes()), &validation)
}
