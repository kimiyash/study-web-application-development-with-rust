use std::sync::Arc;

use adapter::{
    database::{model::auth, ConnectionPool},
    redis::RedisClient,
    repository::{
        auth::AuthRepositoryImpl, book::BookRespositoryImpl, health::HealthCheckRepositoryImpl,
    },
};
use kernel::repository::{
    auth::AuthRepository, book::BookRespository, health::HealthCheckRepository,
};
use shared::config::AppConfig;

#[derive(Clone)]
pub struct AppRegistry {
    health_check_repository: Arc<dyn HealthCheckRepository>,
    book_repository: Arc<dyn BookRespository>,
    auth_repository: Arc<dyn AuthRepository>,
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
        Self {
            health_check_repository,
            book_repository,
            auth_repository,
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
}
