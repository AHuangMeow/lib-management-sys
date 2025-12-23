use crate::constants::{BOOK_ALREADY_EXISTS, COLLECTION_BOOKS, COLLECTION_USERS, USER_NOT_FOUND};
use crate::errors::AppError;
use crate::models::book::Book;
use crate::models::user::User;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::options::ClientOptions;
use mongodb::{Client, Collection, Database};

pub async fn init_mongodb(uri: &str, db_name: &str) -> mongodb::error::Result<Database> {
    let mut client_options = ClientOptions::parse(uri).await?;
    client_options.app_name = Some("ActixAuth".into());
    let client = Client::with_options(client_options)?;
    Ok(client.database(db_name))
}

#[derive(Clone)]
pub struct UserRepository {
    collection: Collection<User>,
}

impl UserRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection::<User>(COLLECTION_USERS),
        }
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        Ok(self.collection.find_one(doc! { "email": email }).await?)
    }

    pub async fn find_by_id(&self, id: &ObjectId) -> Result<Option<User>, AppError> {
        Ok(self.collection.find_one(doc! { "_id": id }).await?)
    }

    pub async fn find_all(&self) -> Result<Vec<User>, AppError> {
        use mongodb::bson::doc;
        let mut cursor = self.collection.find(doc! {}).await?;
        let mut users = Vec::new();
        use futures::stream::TryStreamExt;
        while let Some(user) = cursor.try_next().await? {
            users.push(user);
        }
        Ok(users)
    }

    pub async fn delete_by_id(&self, id: &ObjectId) -> Result<(), AppError> {
        self.collection.delete_one(doc! { "_id": id }).await?;
        Ok(())
    }

    pub async fn set_admin(&self, id: &ObjectId, is_admin: bool) -> Result<(), AppError> {
        self.collection
            .update_one(
                doc! { "_id": id },
                doc! { "$set": { "is_admin": is_admin } },
            )
            .await?;
        Ok(())
    }

    pub async fn create(&self, user: &User) -> Result<(), AppError> {
        self.collection.insert_one(user).await?;
        Ok(())
    }

    pub async fn update_email(&self, id: &ObjectId, new_email: &str) -> Result<(), AppError> {
        self.collection
            .update_one(doc! { "_id": id }, doc! { "$set": { "email": new_email } })
            .await?;
        Ok(())
    }

    pub async fn update_username(&self, id: &ObjectId, new_username: &str) -> Result<(), AppError> {
        self.collection
            .update_one(
                doc! { "_id": id },
                doc! { "$set": { "username": new_username } },
            )
            .await?;
        Ok(())
    }

    pub async fn update_password(
        &self,
        id: &ObjectId,
        password_hash: &str,
    ) -> Result<(), AppError> {
        self.collection
            .update_one(
                doc! { "_id": id },
                doc! { "$set": { "password_hash": password_hash } },
            )
            .await?;
        Ok(())
    }

    pub async fn update_token_version(
        &self,
        id: &ObjectId,
        token_version: i32,
    ) -> Result<(), AppError> {
        self.collection
            .update_one(
                doc! { "_id": id },
                doc! { "$set": { "token_version": token_version } },
            )
            .await?;
        Ok(())
    }

    pub async fn add_borrowed_book(
        &self,
        user_id: &ObjectId,
        book_id: &ObjectId,
    ) -> Result<(), AppError> {
        let mut user = self
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound(USER_NOT_FOUND.into()))?;

        if user.borrowed_books.contains(book_id) {
            return Err(AppError::BadRequest("book already borrowed".into()));
        }

        if user.borrowed_books.len() >= 8 {
            return Err(AppError::BadRequest("borrow limit reached".into()));
        }

        user.borrowed_books.push(book_id.clone());

        self.collection
            .update_one(
                doc! { "_id": user_id },
                doc! { "$set": { "borrowed_books": &user.borrowed_books } },
            )
            .await?;

        Ok(())
    }

    pub async fn remove_borrowed_book(
        &self,
        user_id: &ObjectId,
        book_id: &ObjectId,
    ) -> Result<(), AppError> {
        let result = self
            .collection
            .update_one(
                doc! { "_id": user_id, "borrowed_books": { "$in": [book_id] } },
                doc! { "$pull": { "borrowed_books": book_id } },
            )
            .await?;

        if result.modified_count == 0 {
            return Err(AppError::BadRequest("book not borrowed by user".into()));
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct BookRepository {
    collection: Collection<Book>,
}

impl BookRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection::<Book>(COLLECTION_BOOKS),
        }
    }

    pub async fn create(&self, title: &str, author: &str) -> Result<Book, AppError> {
        if self
            .collection
            .find_one(doc! { "title": title, "author": author })
            .await?
            .is_some()
        {
            return Err(AppError::Conflict(BOOK_ALREADY_EXISTS.into()));
        }

        let book = Book {
            id: ObjectId::new(),
            title: title.to_string(),
            author: author.to_string(),
            stock: 0,
        };

        self.collection.insert_one(&book).await?;
        Ok(book)
    }

    pub async fn find_by_id(&self, id: &ObjectId) -> Result<Option<Book>, AppError> {
        Ok(self.collection.find_one(doc! { "_id": id }).await?)
    }

    pub async fn find_all(&self) -> Result<Vec<Book>, AppError> {
        use futures::stream::TryStreamExt;
        let mut cursor = self.collection.find(doc! {}).await?;
        let mut books = Vec::new();
        while let Some(book) = cursor.try_next().await? {
            books.push(book);
        }
        Ok(books)
    }

    pub async fn delete_by_id(&self, id: &ObjectId) -> Result<(), AppError> {
        self.collection.delete_one(doc! { "_id": id }).await?;
        Ok(())
    }

    pub async fn update_title(&self, id: &ObjectId, title: &str) -> Result<(), AppError> {
        self.collection
            .update_one(doc! { "_id": id }, doc! { "$set": { "title": title } })
            .await?;
        Ok(())
    }

    pub async fn update_author(&self, id: &ObjectId, author: &str) -> Result<(), AppError> {
        self.collection
            .update_one(doc! { "_id": id }, doc! { "$set": { "author": author } })
            .await?;
        Ok(())
    }

    pub async fn update_stock(&self, id: &ObjectId, stock: i32) -> Result<(), AppError> {
        self.collection
            .update_one(doc! { "_id": id }, doc! { "$set": { "stock": stock } })
            .await?;
        Ok(())
    }

    pub async fn borrow_book(&self, id: &ObjectId) -> Result<(), AppError> {
        let result = self
            .collection
            .update_one(
                doc! { "_id": id, "stock": { "$gt": 0 } },
                doc! { "$inc": { "stock": -1 } },
            )
            .await?;

        if result.modified_count == 0 {
            return Err(AppError::BadRequest("no stock available".into()));
        }

        Ok(())
    }

    pub async fn return_book(&self, id: &ObjectId) -> Result<(), AppError> {
        self.collection
            .update_one(doc! { "_id": id }, doc! { "$inc": { "stock": 1 } })
            .await?;
        Ok(())
    }
}
