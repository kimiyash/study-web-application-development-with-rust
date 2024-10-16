use anyhow::Result;
use async_trait::async_trait;
use derive_new::new;
use kernel::model::book::{event::CreateBook, Book};
use kernel::repository::book::BookRespository;
use uuid::Uuid;

use crate::database::ConnectionPool;

#[derive(new)]
pub struct BookRespositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl BookRespository for BookRespositoryImpl {
    async fn create(&self, event: CreateBook) -> Result<()> {
        todo!()
    }

    async fn find_all(&self) -> Result<Vec<Book>> {
        todo!()
    }

    async fn find_by_id(&self, book_id: Uuid) -> Result<Option<Book>> {
        todo!()
    }
}