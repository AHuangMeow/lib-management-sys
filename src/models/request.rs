use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "invalid email format"))]
    pub email: String,
    #[validate(length(min = 3, max = 30, message = "username must be 3-30 characters"))]
    pub username: String,
    #[validate(length(min = 8, message = "password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "invalid email format"))]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateEmailRequest {
    #[validate(email(message = "invalid email format"))]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUsernameRequest {
    #[validate(length(min = 3, max = 30, message = "username must be 3-30 characters"))]
    pub username: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePasswordRequest {
    pub old_password: String,
    #[validate(length(min = 8, message = "password must be at least 8 characters"))]
    pub new_password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email(message = "invalid email format"))]
    pub email: String,
    #[validate(length(min = 3, max = 30, message = "username must be 3-30 characters"))]
    pub username: String,
    #[validate(length(min = 8, message = "password must be at least 8 characters"))]
    pub password: String,
    pub is_admin: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(email(message = "invalid email format"))]
    pub email: Option<String>,
    #[validate(length(min = 3, max = 30, message = "username must be 3-30 characters"))]
    pub username: Option<String>,
    #[validate(length(min = 8, message = "password must be at least 8 characters"))]
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SetRoleRequest {
    pub is_admin: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateBookRequest {
    #[validate(length(min = 1, message = "title must not be empty"))]
    pub title: String,
    #[validate(length(min = 1, message = "author must not be empty"))]
    pub author: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateBookRequest {
    #[validate(length(min = 1, message = "title must not be empty"))]
    pub title: Option<String>,
    #[validate(length(min = 1, message = "author must not be empty"))]
    pub author: Option<String>,
    pub stock: Option<i32>,
}
