use std::sync::Arc;

use adapter::{
    database::{model::{auth, checkout}, ConnectionPool},
    redis::RedisClient,
    repository::{
        auth::AuthRepositoryImpl, book::BookRespositoryImpl, checkout::CheckouRepositoryImpl, health::HealthCheckRepositoryImpl, user::UserRepsitoryImpl
    },
};
use kernel::repository::{
    auth::AuthRepository, book::BookRespository, health::HealthCheckRepository, user::UserRepsitory,
    checkout::CheckouRepository,
};
use shared::config::AppConfig;

#[derive(Clone)]
pub struct AppRegistry {
    health_check_repository: Arc<dyn HealthCheckRepository>,
    book_repository: Arc<dyn BookRespository>,
    auth_repository: Arc<dyn AuthRepository>,
    user_repository: Arc<dyn UserRepsitory>,
    checkout_repository: Arc<dyn CheckouRepository>,
}

impl AppRegistry {
    pub fn new(
        pool: ConnectionPool,
        redis_client: Arc<RedisClient>,
        app_config: AppConfig,
    ) -> Self {
        let health_check_repository = Arc::new(HealthCheckRepositoryImpl::new(pool.clone()));
        let book_repository = Arc::new(BookRespositoryImpl::new(pool.clone()));
        let auth_repository = Arc::new(AuthRepositoryImpl::new(
            pool.clone(),
            redis_client.clone(),
            app_config.auth.ttl,
        ));
        let user_repository = Arc::new(UserRepsitoryImpl::new(pool.clone()));
        let checkout_repository = Arc::new(CheckouRepositoryImpl::new(pool.clone()));
        Self {
            health_check_repository,
            book_repository,
            auth_repository,
            user_repository,
            checkout_repository,
        }
    }

    pub fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository> {
        self.health_check_repository.clone()
    }

    pub fn book_repository(&self) -> Arc<dyn BookRespository> {
        self.book_repository.clone()
    }

    pub fn auth_repository(&self) -> Arc<dyn AuthRepository> {
        self.auth_repository.clone()
    }

    pub fn user_repository(&self) -> Arc<dyn UserRepsitory> {
        self.user_repository.clone()
    }

    pub fn checked_repository(&self) -> Arc<dyn CheckouRepository> {
        self.checkout_repository.clone()
    }
}
