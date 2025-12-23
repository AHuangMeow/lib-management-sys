pub const COLLECTION_USERS: &str = "users";
pub const COLLECTION_BOOKS: &str = "books";

pub const DEFAULT_JWT_EXP_HOURS: i64 = 24;
pub const MIN_JWT_SECRET_LENGTH: usize = 32;

pub const DEFAULT_HOST: &str = "0.0.0.0";
pub const DEFAULT_PORT: &str = "8080";

pub const REGISTER_SUCCESS: &str = "successfully registered";
pub const LOGIN_SUCCESS: &str = "successfully logged in";
pub const LOGOUT_SUCCESS: &str = "successfully logged out";
pub const TOKEN_BLACKLISTED: &str = "token has been blacklisted";
pub const PROFILE_FETCHED: &str = "successfully fetched user profile";
pub const EMAIL_UPDATED: &str = "successfully updated email";
pub const USERNAME_UPDATED: &str = "successfully updated username";
pub const PASSWORD_UPDATED: &str = "successfully updated password";
pub const USER_INFO_FETCHED: &str = "successfully fetched user info";
pub const USER_INFOS_FETCHED: &str = "successfully fetched user infos";
pub const USER_CREATED: &str = "successfully created user";
pub const USER_UPDATED: &str = "successfully updated user";
pub const USER_DELETED: &str = "successfully deleted user";
pub const USER_SET_AS_ADMIN: &str = "successfully set user as admin";
pub const ADMIN_SET_AS_USER: &str = "successfully set admin as user";

pub const BOOK_CREATED: &str = "successfully created book";
pub const BOOK_UPDATED: &str = "successfully updated book";
pub const BOOK_DELETED: &str = "successfully deleted book";
pub const BOOK_INFO_FETCHED: &str = "successfully fetched book info";
pub const BOOKS_FETCHED: &str = "successfully fetched books";
pub const BOOK_BORROWED: &str = "successfully borrowed book";
pub const BOOK_RETURNED: &str = "successfully returned book";
pub const BOOK_NOT_FOUND: &str = "book not found";

pub const EMAIL_ALREADY_EXISTS: &str = "email already registered";
pub const BOOK_ALREADY_EXISTS: &str = "book already exists";
pub const INVALID_CREDENTIALS: &str = "invalid username or password";
pub const INVALID_OLD_PASSWORD: &str = "invalid old password";
pub const USER_NOT_FOUND: &str = "user not found";
pub const AUTH_REQUIRED: &str = "authentication required";
pub const INVALID_USER_ID: &str = "invalid user id";
pub const PERMISSION_DENIED: &str = "permission denied";
pub const INTERNAL_SERVER_ERROR: &str = "internal server error";

pub const MONGO_URI: &str = "MONGO_URI";
pub const MONGO_DB: &str = "MONGO_DB";
pub const JWT_SECRET: &str = "JWT_SECRET";
pub const JWT_EXP_HOURS: &str = "JWT_EXP_HOURS";
pub const APP_HOST: &str = "APP_HOST";
pub const APP_PORT: &str = "APP_PORT";
pub const REDIS_URI: &str = "REDIS_URI";
pub const SSL_CERT_PATH: &str = "SSL_CERT_PATH";
pub const SSL_KEY_PATH: &str = "SSL_KEY_PATH";
