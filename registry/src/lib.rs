use std::sync::Arc;

use adapter::{
    database::{
        model::{auth, checkout},
        ConnectionPool,
    },
    redis::RedisClient,
    repository::{
        auth::AuthRepositoryImpl, book::BookRespositoryImpl, checkout::CheckouRepositoryImpl,
        health::HealthCheckRepositoryImpl, user::UserRepsitoryImpl,
    },
};
use kernel::repository::{
    auth::AuthRepository, book::BookRepository, checkout::CheckouRepository,
    health::HealthCheckRepository, user::UserRepository,
};
use mockall::predicate::*;
use mockall::*;
use shared::config::AppConfig;

#[derive(Clone)]
pub struct AppRegistryImpl {
    health_check_repository: Arc<dyn HealthCheckRepository>,
    book_repository: Arc<dyn BookRepository>,
    auth_repository: Arc<dyn AuthRepository>,
    user_repository: Arc<dyn UserRepository>,
    checkout_repository: Arc<dyn CheckouRepository>,
}

impl AppRegistryImpl {
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
}

#[mockall::automock]
pub trait AppRegistryExt {
    fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository>;
    fn book_repository(&self) -> Arc<dyn BookRepository>;
    fn auth_repository(&self) -> Arc<dyn AuthRepository>;
    fn user_repository(&self) -> Arc<dyn UserRepository>;
    fn checkout_repository(&self) -> Arc<dyn CheckouRepository>;
}

impl AppRegistryExt for AppRegistryImpl {
    fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository> {
        self.health_check_repository.clone()
    }

    fn book_repository(&self) -> Arc<dyn BookRepository> {
        self.book_repository.clone()
    }

    fn auth_repository(&self) -> Arc<dyn AuthRepository> {
        self.auth_repository.clone()
    }

    fn user_repository(&self) -> Arc<dyn UserRepository> {
        self.user_repository.clone()
    }

    fn checkout_repository(&self) -> Arc<dyn CheckouRepository> {
        self.checkout_repository.clone()
    }
}

pub type AppRegistry = Arc<dyn AppRegistryExt + Send + Sync + 'static>;
