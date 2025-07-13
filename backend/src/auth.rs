use actix_web::{dev::Payload, Error, FromRequest, HttpRequest};
use actix_web::http::header::AUTHORIZATION;
use futures::future::{ready, Ready};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, ConnectionTrait, ActiveModelTrait};
use std::env;
use bcrypt::{hash, verify, DEFAULT_COST};

use crate::models::{Claims, CreateUserRequest, Entity as UserEntity, Model as UserModel, Role, ActiveModel as UserActiveModel};

pub struct AuthService;

impl AuthService {
    pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
        hash(password, DEFAULT_COST)
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
        verify(password, hash)
    }

    pub fn create_jwt(claims: &Claims) -> Result<String, jsonwebtoken::errors::Error> {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        encode(&Header::default(), claims, &EncodingKey::from_secret(secret.as_ref()))
    }

    pub fn decode_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )?;
        Ok(token_data.claims)
    }

    pub async fn authenticate_user(
        db: &DatabaseConnection,
        email: &str,
        password: &str,
    ) -> Result<UserModel, Box<dyn std::error::Error>> {
        let user = UserEntity::find()
            .filter(<UserEntity as sea_orm::EntityTrait>::Column::Email.eq(email))
            .one(db)
            .await?
            .ok_or("User not found")?;

        let is_valid = Self::verify_password(password, &user.password_hash)?;
        if !is_valid {
            return Err("Invalid password".into());
        }

        Ok(user)
    }

    pub async fn create_user(
        db: &DatabaseConnection,
        user_data: CreateUserRequest,
    ) -> Result<UserModel, Box<dyn std::error::Error>> {
        // Check if role is valid
        let role = Role::from_str(&user_data.role)
            .ok_or("Invalid role")?;

        // Hash password
        let password_hash = Self::hash_password(&user_data.password)?;

        // Create user
        let user = UserModel {
            id: 0, // Will be set by database
            email: user_data.email,
            password_hash,
            role: role.as_str().to_string(),
            created_at: chrono::Utc::now().into(),
            updated_at: chrono::Utc::now().into(),
        };

        let user_active: UserActiveModel = user.clone().into();
        let result = UserEntity::insert(user_active).exec(db).await?;
        
        Ok(UserModel {
            id: result.last_insert_id,
            ..user
        })
    }

    pub async fn get_current_user(
        db: &DatabaseConnection,
        user_id: i32,
    ) -> Result<UserModel, Box<dyn std::error::Error>> {
        let user = UserEntity::find_by_id(user_id)
            .one(db)
            .await?
            .ok_or("User not found")?;
        Ok(user)
    }
}

// Middleware for extracting user from request
pub struct AuthenticatedUser {
    pub claims: Claims,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let auth_header = req.headers().get(AUTHORIZATION);
        
        match auth_header {
            Some(header) => {
                let token = header.to_str().unwrap_or("").replace("Bearer ", "");
                if token.is_empty() {
                    return ready(Err(actix_web::error::ErrorUnauthorized("No token provided")));
                }

                match AuthService::decode_jwt(&token) {
                    Ok(claims) => ready(Ok(AuthenticatedUser { claims })),
                    Err(_) => ready(Err(actix_web::error::ErrorUnauthorized("Invalid token"))),
                }
            }
            None => ready(Err(actix_web::error::ErrorUnauthorized("No authorization header"))),
        }
    }
}

// Role-based authorization middleware
pub fn require_role(required_role: &str) -> impl Fn(&Claims) -> bool {
    let required_role = required_role.to_string();
    move |claims: &Claims| {
        match required_role.as_str() {
            "admin" => claims.role == "admin",
            "teacher" => claims.role == "teacher" || claims.role == "admin",
            "student" => claims.role == "student" || claims.role == "teacher" || claims.role == "admin",
            _ => false,
        }
    }
} 