use crate::TypeMapKey;
use jsonwebtoken::{DecodingKey, EncodingKey};
use meilisearch_sdk::indexes::Index;
use s3::Bucket;
use sea_orm::DatabaseConnection;

pub struct Database;
pub struct SearchIndex;
pub struct S3Bucket;
pub struct EKey;
pub struct DKey;

impl TypeMapKey for Database {
    type Value = DatabaseConnection;
}

impl TypeMapKey for SearchIndex {
    type Value = Index;
}

impl TypeMapKey for S3Bucket {
    type Value = Bucket;
}

impl TypeMapKey for EKey {
    type Value = EncodingKey;
}

impl TypeMapKey for DKey {
    type Value = DecodingKey;
}
