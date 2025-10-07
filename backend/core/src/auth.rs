use crate::{error::CryptoTradeError, models::User, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use totp_rs::{Algorithm, TOTP};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // User ID
    pub email: String,
    pub username: String,
    pub exp: i64,
    pub iat: i64,
    pub role: String,
}

#[derive(Clone)]
pub struct AuthService {
    jwt_secret: String,
    jwt_expiration: i64,
}

impl AuthService {
    pub fn new(jwt_secret: String, jwt_expiration: i64) -> Self {
        Self {
            jwt_secret,
            jwt_expiration,
        }
    }

    pub fn hash_password(&self, password: &str) -> Result<String> {
        hash(password, DEFAULT_COST).map_err(Into::into)
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        verify(password, hash).map_err(Into::into)
    }

    pub fn generate_jwt(&self, user: &User) -> Result<String> {
        let now = Utc::now();
        let expiration = now + Duration::seconds(self.jwt_expiration);

        let claims = Claims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            username: user.username.clone(),
            exp: expiration.timestamp(),
            iat: now.timestamp(),
            role: "user".to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(Into::into)
    }

    pub fn verify_jwt(&self, token: &str) -> Result<TokenData<Claims>> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|e| CryptoTradeError::Authentication {
            message: format!("Invalid JWT token: {}", e),
        })
    }

    pub fn generate_refresh_token(&self, user_id: Uuid) -> Result<String> {
        let now = Utc::now();
        let expiration = now + Duration::days(30); // 30 days for refresh token

        let claims = Claims {
            sub: user_id.to_string(),
            email: "".to_string(), // Don't include sensitive info in refresh token
            username: "".to_string(),
            exp: expiration.timestamp(),
            iat: now.timestamp(),
            role: "refresh".to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(Into::into)
    }

    pub fn generate_2fa_secret(&self) -> String {
        self.generate_totp_secret()
    }

    pub fn generate_totp_secret(&self) -> String {
        use base32::Alphabet;
        let secret: Vec<u8> = (0..32).map(|_| rand::random::<u8>()).collect();
        base32::encode(Alphabet::Rfc4648 { padding: false }, &secret)
    }

    pub fn generate_backup_codes(&self) -> Vec<String> {
        (0..8)
            .map(|_| format!("{:08}", rand::random::<u32>() % 100000000))
            .collect()
    }

    pub fn generate_qr_code(&self, secret: &str, identifier: &str) -> Result<String> {
        let totp_url = self.generate_totp_url(secret, identifier, "CryptoTrade Exchange")?;
        // In a real implementation, you would generate an actual QR code image
        // For now, just return the TOTP URL
        Ok(totp_url)
    }

    pub fn generate_totp_url(&self, secret: &str, email: &str, issuer: &str) -> Result<String> {
        Ok(format!(
            "otpauth://totp/{}:{}?secret={}&issuer={}",
            issuer, email, secret, issuer
        ))
    }

    pub fn verify_2fa_code(&self, secret: &str, token: &str) -> Result<bool> {
        self.verify_totp(secret, token)
    }

    pub fn verify_totp(&self, secret: &str, token: &str) -> Result<bool> {
        let totp = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            secret.as_bytes().to_vec(),

        )
        .map_err(|_| CryptoTradeError::Internal)?;

        Ok(totp.check_current(token).unwrap_or(false))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
    pub token_type: String,
}

impl AuthService {
    pub fn verify_refresh_token(&self, token: &str) -> Result<TokenData<RefreshTokenClaims>> {
        decode::<RefreshTokenClaims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|e| CryptoTradeError::Authentication {
            message: format!("Invalid refresh token: {}", e),
        })
    }
}
