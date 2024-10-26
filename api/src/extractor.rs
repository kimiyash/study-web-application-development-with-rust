use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::{async_trait, RequestPartsExt};
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use kernel::model::auth::AccessToken;
use kernel::model::id::UserId;
use kernel::model::role::Role;
use kernel::model::user::User;
use shared::error::AppError;

use registry::AppRegistry;

// リクエストの前処理を実行後、handler に渡す構造体を定義
pub struct AuthrizedUser {
    pub access_token: AccessToken,
    pub user: User,
}

impl AuthrizedUser {
    pub fn id(&self) -> UserId {
        self.user.id
    }

    pub fn is_admin(&self) -> bool {
        self.user.role == Role::Admin
    }
}

#[async_trait]
impl FromRequestParts<AppRegistry> for AuthrizedUser {
    type Rejection = AppError;

    // handler メソッドの引数に AuthorizedUser を追加したときはこのメソッドが呼ばれる。
    async fn from_request_parts(
        parts: &mut Parts,
        registry: &AppRegistry,
    ) -> Result<Self, Self::Rejection> {
        // HTTP ヘッダからアクセストークンを取り出す。
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::UnauthenticatedError)?;
        let access_token = AccessToken(bearer.token().to_string());

        // アクセストークンが紐づくユーザーIDを抽出する
        let user_id = registry
            .auth_repository()
            .fetch_user_id_from_token(&access_token)
            .await?
            .ok_or(AppError::UnauthenticatedError)?;

        // ユーザーIDでデータベースからユーザーのレコードを引く
        let user = registry
            .user_repository()
            .find_current_user(user_id)
            .await?
            .ok_or(AppError::UnauthorizedError)?;

        Ok(Self { access_token, user })
    }
}