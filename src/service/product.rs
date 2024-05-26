use crate::provider::DatabaseProvider;
use crate::model::product;

use super::{ServiceError, ServiceResult};

pub type ProductId = i64;

pub trait ProductService {
    async fn create_product(&self, product: product::Product) -> ServiceResult<ProductId>;
    async fn init_service(&self) -> ServiceResult<()>;
}

impl ProductService for DatabaseProvider {
    async fn create_product(&self, product: product::Product) -> ServiceResult<ProductId> {
        let acquire_result = self.db.acquire().await;
        let Ok(mut conn) = acquire_result else {
            log::error!("Error opening db {:?}", acquire_result);
            return Err(ServiceError::ProviderError);
        };

        let create_result = sqlx::query(
            &r#"
            INSERT INTO products (sku) VALUES($1);
        "#,
        )
        .bind(product.details.sku)
        .execute(&mut *conn)
        .await;
        let Ok(id) = create_result else {
            log::error!("Could not create product {:?}", create_result);
            return Err(ServiceError::ProviderError);
        };

        Ok(id.last_insert_rowid())
    }

    async fn init_service(&self) -> Result<(), ServiceError> {
        let acquire_result = self.db.acquire().await;
        let Ok(mut conn) = acquire_result else {
            log::error!("Error opening db {:?}", acquire_result);
            return Err(ServiceError::ProviderError);
        };

        let init_result = sqlx::query(
            &r#"
            CREATE TABLE IF NOT EXISTS products (
                ID INTEGER NOT NULL UNIQUE PRIMARY KEY,
                sku TEXT NOT NULL
            );
        "#,
        )
        .execute(&mut *conn)
        .await;

        match init_result {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("Error creating table 'products' {:?}", e);
                Err(())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::ProductService;
    use crate::{model::product::{self, Product}, provider::DatabaseProvider};

    #[tokio::test]
    async fn test_service_init() {
        let result = DatabaseProvider::new_sqlite("test.db".to_string()).await;
        let Ok(db_provider) = result else {
            panic!("failed to build sqlite db_provider {:?}", result);
        };
        let init_result = db_provider.init_service().await;
        match init_result {
            Ok(_) => {}
            Err(_) => {
                panic!("error inititalizing product service for sqlite database provider");
            }
        }
    }

    #[tokio::test]
    async fn test_create_product() {
        let result = DatabaseProvider::new_sqlite("test.db".to_string()).await;
        let Ok(db_provider) = result else {
            panic!("failed to build sqlite db_provider {:?}", result);
        };
        let _init_result = db_provider.init_service().await;
        let pricing = product::ProductPricing{
            price: product::Cents(1),
        };
        let details = product::ProductDetails{
            sku: String::from("1"),
            description: String::from("nail"),
        };
        let product = product::Product{pricing, details};
        let create_result = db_provider.create_product(product).await;
        match create_result {
            Ok(_id) => {}
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}
