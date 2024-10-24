use kernel::model::book::Book;
use kernel::model::id::BookId;

pub struct BookRow {
    pub book_id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}

impl From<BookRow> for Book {
    fn from(value: BookRow) -> Self {
        // パターンマッチをつかって BookRow の中身を取り出す
        let BookRow {
            book_id,
            title,
            author,
            isbn,
            description,
        } = value;
        Self {
            id: book_id,
            title,
            author,
            isbn,
            description,
        }
    }
}
