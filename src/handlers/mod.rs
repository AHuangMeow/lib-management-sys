mod admin;
mod auth;
mod health;
mod user;
mod book;

pub use admin::admin_scope;
pub use auth::auth_scope;
pub use health::health_check;
pub use user::user_scope;
pub use book::book_scope;
