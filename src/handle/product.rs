use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::{model::product, service::ProductService};

pub struct ProductHandler;

#[derive(Clone)]
pub struct ProductServiceState<T: ProductService> {
    service: T,
}

#[derive(Deserialize)]
pub struct CreateProduct {
    sku: String,
}

#[derive(Serialize)]
pub struct Product {
    id: i64,
    sku: String,
}

impl ProductHandler {
    pub async fn create_product<T: ProductService>(
        State(product_service): State<ProductServiceState<T>>,
        Json(payload): Json<product::Product>,
    ) -> Result<(StatusCode, Json<Product>), StatusCode> {
        let result = product_service.service.create_product(payload.clone()).await;
        match result {
            Ok(id) => Ok((StatusCode::ACCEPTED, Json(Product{id, sku: payload.details.sku}))),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

#[cfg(test)]
mod test {
    use axum::{routing::post, Router};

    use super::{ProductHandler, ProductServiceState};
    use crate::{provider::DatabaseProvider, service::ProductService};

    #[tokio::test]
    async fn test_route_attachment() {
        let Ok(db_provider) = DatabaseProvider::new_sqlite("test.db".to_string()).await else {
            panic!("Error building database provider");
        };
        let Ok(_) = db_provider.init_service().await else {
            panic!("Error initializing product service on database provider");
        };
        let _app: Router<()> = Router::new()
            .route("/user", post(ProductHandler::create_product))
            .with_state(ProductServiceState {
                service: db_provider,
            });
    }
}
