#[cfg(test)]
mod tests {
    #[sqlx::test]
    async fn it_works(pool: sqlx::PgPool) {
        // 接続確認
        let row = sqlx::query!("SELECT 1 + 1 AS result")
            .fetch_one(&pool)
            .await
            .unwrap();
        let result = row.result;
        assert_eq!(result, Some(2))
    }

    #[sqlx::test(fixtures("common"))]
    async fn it_works2(pool: sqlx::PgPool) {
        // 接続確認
        let row = sqlx::query!("SELECT author FROM books WHERE title = 'Test Book 1'")
            .fetch_one(&pool)
            .await
            .unwrap();
        let result = row.author;
        assert_eq!(result, "Test Author 1".to_string());
    }
}
