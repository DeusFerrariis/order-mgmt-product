use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductDetails {
    pub sku: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cents(pub u64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductPricing {
    pub price: Cents,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub pricing: ProductPricing,
    pub details: ProductDetails
}