use actix_web::{web, HttpResponse, Result};
use sea_orm::DatabaseConnection;
use serde_json::json;

use crate::auth::{AuthService, AuthenticatedUser};
use crate::models::{CreateUserRequest, LoginRequest, LoginResponse, UserResponse, Claims};

pub async fn login(
    db: web::Data<DatabaseConnection>,
    login_data: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    match AuthService::authenticate_user(&db, &login_data.email, &login_data.password).await {
        Ok(user) => {
            let claims = Claims {
                sub: user.id,
                email: user.email.clone(),
                role: user.role.clone(),
                exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
            };

            match AuthService::create_jwt(&claims) {
                Ok(token) => {
                    let response = LoginResponse {
                        token,
                        user: UserResponse {
                            id: user.id,
                            email: user.email,
                            role: user.role,
                        },
                    };
                    Ok(HttpResponse::Ok().json(response))
                }
                Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
                    "error": "Failed to create token"
                }))),
            }
        }
        Err(_) => Ok(HttpResponse::Unauthorized().json(json!({
            "error": "Invalid credentials"
        }))),
    }
}

pub async fn register(
    db: web::Data<DatabaseConnection>,
    user_data: web::Json<CreateUserRequest>,
) -> Result<HttpResponse> {
    match AuthService::create_user(&db, user_data.into_inner()).await {
        Ok(user) => {
            let response = UserResponse {
                id: user.id,
                email: user.email,
                role: user.role,
            };
            Ok(HttpResponse::Created().json(response))
        }
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "error": e.to_string()
        }))),
    }
}

pub async fn get_current_user(
    db: web::Data<DatabaseConnection>,
    auth_user: AuthenticatedUser,
) -> Result<HttpResponse> {
    match AuthService::get_current_user(&db, auth_user.claims.sub).await {
        Ok(user) => {
            let response = UserResponse {
                id: user.id,
                email: user.email,
                role: user.role,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(_) => Ok(HttpResponse::NotFound().json(json!({
            "error": "User not found"
        }))),
    }
}

pub async fn protected_route(
    auth_user: AuthenticatedUser,
) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "message": "This is a protected route",
        "user": {
            "id": auth_user.claims.sub,
            "email": auth_user.claims.email,
            "role": auth_user.claims.role,
        }
    })))
} 