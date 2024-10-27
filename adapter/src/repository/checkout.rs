use std::f32::consts::E;

use crate::database::{
    model::checkout::{CheckoutRow, CheckoutStateRow, ReturnedCheckoutRow},
    ConnectionPool,
};
use async_trait::async_trait;
use derive_new::new;
use kernel::model::{auth::event, checkout::{
    event::{CreateCheckout, UpdateReturned},
    Checkout,
}};
use kernel::model::id::{BookId, CheckoutId, UserId};
use kernel::repository::checkout::CheckouRepository;
use shared::error::{AppError, AppResult};

#[derive(new)]
pub struct CheckouRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl CheckouRepository for CheckouRepositoryImpl {
    async fn create(&self, event: CreateCheckout) -> AppResult<()> {
        let mut tx = self.db.begin().await?;

        // トランザクション分離レベルを SERIALIZABLE に設定
        self.set_transaction_serializable(&mut tx).await?;

        // 事前のチェックとして、以下をしらべる
        // - 指定の蔵書IDを持つ蔵書が存在するか
        // - 存在した場合、この蔵書は貸出中ではないか？
        {
            let res = sqlx::query_as!(
                CheckoutStateRow,
                r#"
                    SELECT
                        b.book_id,
                        c.checkout_id AS "checkout_id?: CheckoutId",
                        NULL AS "user_id?: UserId"
                    FROM books AS b
                    LEFT OUTER JOIN checkouts AS c USING(book_id)
                    WHERE b.book_id = $1;
                "#,
                event.book_id as _
            )
            .fetch_optional(&mut *tx)
            .await
            .map_err(AppError::SpecificOperationError)?;

            match res {
                // 指定した書籍が存在しない場合
                None => {
                    return Err(AppError::EntityNotFound(format!(
                        "書籍 ({}) が見つかりませんでした",
                        event.book_id
                    )))
                }
                // 指定した書籍が存在するが貸出中の場合
                Some(CheckoutStateRow {
                    checkout_id: Some(_),
                    ..
                }) =>  {
                    return Err(AppError::UnprocessableEntity(format!(
                        "書籍 ({}) に対する貸出がすでに存在してます",
                        event.book_id
                    )))
                }
                _ => {} // それ以外は処理続行
            }
        }

        // 貸出処理を行う、つまり checkouts テーブルにレコードを追加する
        let checkout_id = CheckoutId::new();
        let res = sqlx::query!(
            r#"
                INSERT INTO checkouts
                (checkout_id, book_id, user_id, checked_out_at)
                VALUES ($1, $2, $3, $4)
                ;
            "#,
            checkout_id as _,
            event.book_id as _,
            event.checked_out_by as _,
            event.checked_out_at,
        )
        .execute(&mut *tx)
        .await
        .map_err(AppError::SpecificOperationError)?;

        if res.rows_affected() < 1 {
            return Err(AppError::NoRowsAffectedError(
                "No checkout record has been created".into(),
            ))
        }

        tx.commit().await.map_err(AppError::TransactionError)?;

        Ok(())
    }

    async fn update_returned(&self, event: UpdateReturned) -> AppResult<()> {
        let mut tx = self.db.begin().await?;

        // トランザクション分離レベルを SERIALIZABLE に設定
        self.set_transaction_serializable(&mut tx).await?;

        // 返却操作時は事前のチェックとして、以下をしらべる
        // - 指定の蔵書IDをもつ蔵書が存在するか
        // - 存在した場合
        //  - この蔵書は貸出中であり
        //  - かつ、借りたユーザーが指定のユーザーと同じか
        //
        // 上記の両方がYesだった場合、このブロック以降の処理にすすむ。
        // なお、ブロックの仕様は意図的である。こうすることで、
        // res 変数がシャドーイングで上書きされるのを防ぐなどの
        // メリットがある
        {
            let res = sqlx::query_as!(
                CheckoutStateRow,
                r#"
                    SELECT
                        b.book_id,
                        c.checkout_id AS "checkout_id?: CheckoutId",
                        c.user_id AS "user_id?: UserId"
                    FROM books AS b
                    LEFT OUTER JOIN checkouts AS c USING(book_id)
                    WHERE book_id = $1;
                "#,
                event.book_id as _,
            )
            .fetch_optional(&mut *tx)
            .await
            .map_err(AppError::SpecificOperationError)?;

            match res {
                // 指定した書籍がそもそも存在しない場合
                None => {
                    return Err(AppError::EntityNotFound(format!(
                        "書籍 ({}) が見つかりませんでした",
                        event.book_id
                    )))
                }
                // 指定した書籍が貸出中であり、貸出IDまたは借りたユーザーが異なる場合
                Some(CheckoutStateRow {
                    checkout_id: Some(c),
                    user_id: Some(u),
                    ..
                }) if (c, u) != (event.checkout_id, event.returned_by) => {
                    return Err(AppError::UnprocessableEntity(format!(
                        "指定の書籍 (ID ({}), ユーザー ({}), 書籍 ({})) は返却できません",
                        event.checkout_id,
                        event.returned_by,
                        event.book_id
                    )))
                }
                _ => {} // それ以外は処理続行
            }
        }

        // データベス上の返却操作として
        // checkouts テーブルにある該当貸出IDのレコードを、
        // returned_at を追加して returned_checkouts テーブルに INSERT する
        let res = sqlx::query!(
            r#"
                INSERT INTO returned_checkouts
                (checkout_id, book_id, user_id, checked_out_at, returned_at)
                SELECT checkout_id, book_id, user_id, checked_out_at, $2
                FROM checkouts
                WHERE checkout_id = $1
                ;
            "#,
            event.checkout_id as _,
            event.returned_at,
        )
        .execute(&mut *tx)
        .await
        .map_err(AppError::SpecificOperationError)?;

        if res.rows_affected() < 1 {
            return Err(AppError::NoRowsAffectedError(
                "No returning record has been updated".into(),
            ))
        }

        // 上記処理が成功したら　checkouts テーブルから該当貸出IDのレコードを削除する
        let res = sqlx::query!(
            r#"
                DELETE FROM checkouts WHERE checkout_id = $1;
            "#,
            event.checkout_id as _,
        )
        .execute(&mut *tx)
        .await
        .map_err(AppError::SpecificOperationError)?;

        if res.rows_affected() < 1 {
            return Err(AppError::NoRowsAffectedError(
                "No checkout record has been deleted".into(),
            ));
        }

        tx.commit().await.map_err(AppError::TransactionError)?;

        Ok(())
    }

    async fn find_unreturned_all(&self) -> AppResult<Vec<Checkout>> {

        todo!()
    }

    async fn find_unreturned_by_user_id(&self, user_id: UserId) -> AppResult<Vec<Checkout>> {

        todo!()
    }

    async fn find_history_by_book_id(&self, book_id: BookId) -> AppResult<Vec<Checkout>>{

        todo!()
    }
}