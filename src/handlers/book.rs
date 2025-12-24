use crate::auth::AuthenticatedUser;
use crate::constants::*;
use crate::database::mongodb::{BookRepository, UserRepository};
use crate::errors::AppError;
use crate::models::response::{BookDetail, BookInfo, Response};
use actix_web::web::{scope, Data, Path};
use actix_web::{get, post, HttpResponse};
use mongodb::bson::oid::ObjectId;

#[get("")]
async fn get_all_books(book_repo: Data<BookRepository>) -> Result<HttpResponse, AppError> {
    let books = book_repo.find_all().await?;

    let infos: Vec<BookInfo> = books
        .into_iter()
        .map(|b| BookInfo {
            id: b.id.to_hex(),
            title: b.title,
            author: b.author,
            stock: b.stock,
        })
        .collect();

    Ok(HttpResponse::Ok().json(Response {
        msg: BOOKS_FETCHED.into(),
        data: Some(infos),
    }))
}

#[get("/title/{title}")]
async fn get_books_by_title(
    book_repo: Data<BookRepository>,
    title: Path<String>,
) -> Result<HttpResponse, AppError> {
    let books = book_repo.find_by_title(title.as_str()).await?;
    let infos: Vec<BookInfo> = books
        .into_iter()
        .map(|b| BookInfo {
            id: b.id.to_hex(),
            title: b.title,
            author: b.author,
            stock: b.stock,
        })
        .collect();

    Ok(HttpResponse::Ok().json(Response {
        msg: BOOKS_FETCHED.into(),
        data: Some(infos),
    }))
}

#[get("/author/{author}")]
async fn get_books_by_author(
    book_repo: Data<BookRepository>,
    author: Path<String>,
) -> Result<HttpResponse, AppError> {
    let books = book_repo.find_by_author(author.as_str()).await?;
    let infos: Vec<BookInfo> = books
        .into_iter()
        .map(|b| BookInfo {
            id: b.id.to_hex(),
            title: b.title,
            author: b.author,
            stock: b.stock,
        })
        .collect();

    Ok(HttpResponse::Ok().json(Response {
        msg: BOOKS_FETCHED.into(),
        data: Some(infos),
    }))
}

#[get("/id/{id}")]
async fn get_book_by_id(
    book_repo: Data<BookRepository>,
    id: Path<String>,
) -> Result<HttpResponse, AppError> {
    let object_id = ObjectId::parse_str(id.as_str()).map_err(|_| {
        AppError::BadRequest(INVALID_BOOK_ID.into())
    })?;

    let book = book_repo
        .find_by_id(&object_id)
        .await?
        .ok_or_else(|| AppError::NotFound(BOOK_NOT_FOUND.into()))?;

    Ok(HttpResponse::Ok().json(Response {
        msg: BOOK_INFO_FETCHED.into(),
        data: Some(BookDetail {
            id: book.id.to_hex(),
            title: book.title,
            author: book.author,
            stock: book.stock,
        }),
    }))
}

#[post("/borrow/{id}")]
async fn borrow_book(
    user: AuthenticatedUser,
    book_repo: Data<BookRepository>,
    user_repo: Data<UserRepository>,
    id: Path<String>,
) -> Result<HttpResponse, AppError> {
    let object_id = ObjectId::parse_str(id.as_str()).map_err(|_| {
        AppError::BadRequest(INVALID_BOOK_ID.into())
    })?;

    let user_id = ObjectId::parse_str(&user.user_id).map_err(|_| {
        AppError::BadRequest(INVALID_USER_ID.into())
    })?;

    book_repo.borrow_book(&object_id).await?;

    if let Err(e) = user_repo.add_borrowed_book(&user_id, &object_id).await {
        if let Err(rollback_err) = book_repo.return_book(&object_id).await {
            tracing::error!("failed to rollback book stock after user borrow failure: {:?}", rollback_err);
        }
        return Err(e);
    }

    Ok(HttpResponse::Ok().json(Response::<()> {
        msg: BOOK_BORROWED.into(),
        data: None,
    }))
}

#[post("/return/{id}")]
async fn return_book(
    user: AuthenticatedUser,
    book_repo: Data<BookRepository>,
    user_repo: Data<UserRepository>,
    id: Path<String>,
) -> Result<HttpResponse, AppError> {
    let object_id = ObjectId::parse_str(id.as_str()).map_err(|_| {
        AppError::BadRequest(INVALID_BOOK_ID.into())
    })?;

    let user_id = ObjectId::parse_str(&user.user_id).map_err(|_| {
        AppError::BadRequest(INVALID_USER_ID.into())
    })?;

    user_repo.remove_borrowed_book(&user_id, &object_id).await?;
    book_repo.return_book(&object_id).await?;

    Ok(HttpResponse::Ok().json(Response::<()> {
        msg: BOOK_RETURNED.into(),
        data: None,
    }))
}

pub fn book_scope() -> actix_web::Scope {
    scope("/books")
        .service(get_all_books)
        .service(get_books_by_title)
        .service(get_books_by_author)
        .service(get_book_by_id)
        .service(borrow_book)
        .service(return_book)
}
