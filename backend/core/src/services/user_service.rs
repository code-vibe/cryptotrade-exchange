use crate::{
    auth::AuthService,
    database::Database,
    error::CryptoTradeError,
    models::*,
    Result,
};
use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

#[derive(Clone)]
pub struct UserService {
    db: Database,
    auth_service: AuthService,
}

impl UserService {
    pub fn new(db: Database, auth_service: AuthService) -> Self {
        Self { db, auth_service }
    }

    pub async fn register(&self, request: RegisterRequest) -> Result<AuthResponse> {
        request.validate().map_err(|e| CryptoTradeError::Validation {
            message: e.to_string(),
        })?;

        // Check if user already exists
        let existing_user = sqlx::query("SELECT id FROM users WHERE email = $1 OR username = $2")
            .bind(&request.email)
            .bind(&request.username)
            .fetch_optional(&self.db)
            .await?;

        if existing_user.is_some() {
            return Err(CryptoTradeError::Validation {
                message: "User with this email or username already exists".to_string(),
            });
        }

        let password_hash = self.auth_service.hash_password(&request.password)?;
        let user_id = Uuid::new_v4();

        // Create user
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (id, email, username, password_hash, first_name, last_name, is_verified, is_active, two_fa_enabled, kyc_status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, false, true, false, 'pending', $7, $7)
            RETURNING *
            "#
        )
        .bind(user_id)
        .bind(&request.email)
        .bind(&request.username)
        .bind(password_hash)
        .bind(&request.first_name)
        .bind(&request.last_name)
        .bind(Utc::now())
        .fetch_one(&self.db)
        .await?;

        // Create default accounts for major currencies
        let currencies = vec!["USD", "BTC", "ETH", "USDT"];
        for currency in currencies {
            self.create_account(user.id, currency).await?;
        }

        let access_token = self.auth_service.generate_jwt(&user)?;
        let refresh_token = self.auth_service.generate_refresh_token(user.id)?;

        Ok(AuthResponse {
            access_token,
            refresh_token,
            expires_in: 3600,
            user: UserProfile {
                id: user.id,
                email: user.email,
                username: user.username,
                first_name: user.first_name,
                last_name: user.last_name,
                is_verified: user.is_verified.unwrap_or(false),
                two_fa_enabled: user.two_fa_enabled.unwrap_or(false),
                kyc_status: user.kyc_status.unwrap_or(KycStatus::Pending),
            },
        })
    }

    pub async fn login(&self, request: LoginRequest) -> Result<AuthResponse> {
        request.validate().map_err(|e| CryptoTradeError::Validation {
            message: e.to_string(),
        })?;

        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1"
        )
        .bind(&request.email)
        .fetch_optional(&self.db)
        .await?
        .ok_or(CryptoTradeError::Authentication {
            message: "Invalid credentials".to_string(),
        })?;

        if !user.is_active.unwrap_or(false) {
            return Err(CryptoTradeError::Authentication {
                message: "Account is disabled".to_string(),
            });
        }

        if !self.auth_service.verify_password(&request.password, &user.password_hash)? {
            return Err(CryptoTradeError::Authentication {
                message: "Invalid credentials".to_string(),
            });
        }

        // Handle 2FA if enabled - use totp_code instead of two_fa_code
        if user.two_fa_enabled.unwrap_or(false) {
            if let Some(code) = &request.totp_code {
                if !self.verify_2fa(user.id, code).await? {
                    return Err(CryptoTradeError::Authentication {
                        message: "Invalid 2FA code".to_string(),
                    });
                }
            } else {
                return Err(CryptoTradeError::TwoFactorRequired);
            }
        }

        let access_token = self.auth_service.generate_jwt(&user)?;
        let refresh_token = self.auth_service.generate_refresh_token(user.id)?;

        Ok(AuthResponse {
            access_token,
            refresh_token,
            expires_in: 3600,
            user: UserProfile {
                id: user.id,
                email: user.email,
                username: user.username,
                first_name: user.first_name,
                last_name: user.last_name,
                is_verified: user.is_verified.unwrap_or(false),
                two_fa_enabled: user.two_fa_enabled.unwrap_or(false),
                kyc_status: user.kyc_status.unwrap_or(KycStatus::Pending),
            },
        })
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_optional(&self.db)
            .await?
            .ok_or(CryptoTradeError::NotFound {
                message: "User not found".to_string(),
            })
    }

    pub async fn enable_2fa(&self, user_id: Uuid) -> Result<TwoFactorResponse> {
        let secret = self.auth_service.generate_2fa_secret();
        let backup_codes = self.auth_service.generate_backup_codes();

        sqlx::query("UPDATE users SET two_fa_secret = $1 WHERE id = $2")
            .bind(&secret)
            .bind(user_id)
            .execute(&self.db)
            .await?;

        let qr_code = self.auth_service.generate_qr_code(&secret, &format!("user_{}", user_id))?;

        Ok(TwoFactorResponse {
            secret,
            qr_code,
            backup_codes,
        })
    }

    pub async fn confirm_2fa(&self, user_id: Uuid, request: ConfirmTwoFactorRequest) -> Result<SuccessResponse> {
        if self.verify_2fa(user_id, &request.code).await? {
            sqlx::query("UPDATE users SET two_fa_enabled = true WHERE id = $1")
                .bind(user_id)
                .execute(&self.db)
                .await?;

            Ok(SuccessResponse {
                message: "Two-factor authentication enabled successfully".to_string(),
            })
        } else {
            Err(CryptoTradeError::Authentication {
                message: "Invalid 2FA code".to_string(),
            })
        }
    }

    pub async fn disable_2fa(&self, user_id: Uuid) -> Result<SuccessResponse> {
        sqlx::query("UPDATE users SET two_fa_enabled = false, two_fa_secret = NULL WHERE id = $1")
            .bind(user_id)
            .execute(&self.db)
            .await?;

        Ok(SuccessResponse {
            message: "Two-factor authentication disabled successfully".to_string(),
        })
    }

    pub async fn create_account(&self, user_id: Uuid, currency: &str) -> Result<Account> {
        let account_id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query_as::<_, Account>(
            "INSERT INTO accounts (id, user_id, currency, balance, available_balance, locked_balance, created_at, updated_at) VALUES ($1, $2, $3, 0, 0, 0, $4, $5) RETURNING *"
        )
        .bind(account_id)
        .bind(user_id)
        .bind(currency)
        .bind(now)
        .bind(now)
        .fetch_one(&self.db)
        .await
        .map_err(Into::into)
    }

    pub async fn get_user_accounts(&self, user_id: Uuid) -> Result<Vec<Account>> {
        sqlx::query_as::<_, Account>(
            "SELECT * FROM accounts WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_all(&self.db)
        .await
        .map_err(Into::into)
    }

    async fn verify_2fa(&self, user_id: Uuid, code: &str) -> Result<bool> {
        let user = self.get_user_by_id(user_id).await?;

        if let Some(secret) = user.two_fa_secret {
            Ok(self.auth_service.verify_2fa_code(&secret, code)?)
        } else {
            Ok(false)
        }
    }
}
