mod product;

pub use product::ProductService;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("failed to initialized service for specified provider")]
    ProviderInitFail,

    #[error("provider rejected operation")]
    ProviderRejection,

    #[error("provider failed to handle operation")]
    ProviderError,

    #[error("service rejected operation")]
    ServiceRejection
}

pub type ServiceResult<T> = Result<T, ServiceError>;