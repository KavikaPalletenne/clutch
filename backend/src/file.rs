use std::fs::File;
use actix_multipart::Multipart;
use bson::doc;
use mongodb::Database;
use uuid::Uuid;
use s3::Bucket;
use crate::models::{FileReference, Resource};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DirectUploadResponse {
    pub url: String,
    pub file_id: String,
}

// Client calls this when upload page loads.
pub fn direct_upload(group_id: String, bucket: &Bucket) -> DirectUploadResponse {
    let file_id = Uuid::new_v4();
    let path = format!("{}/{}", group_id, file_id.clone().to_string());
    let url = bucket.presign_put(path, 86400, None).unwrap();

    DirectUploadResponse {
        url,
        file_id: file_id.to_string(),
    }
}

// After client uploads file, this is called.
pub async fn update_resource_files(resource_id: String, file_name: String, file_size: i64, file_id: String, database: &Database) {
    let query = doc! {
        "_id": resource_id,
    };

    let old_resource = database
        .collection::<Resource>("resources")
        .find_one(query.clone(), None)
        .await
        .expect("Error fetching resource from database");

    if let Some(r) = old_resource {
        let file = FileReference {
            name: file_id,
            title: file_name,
            size: file_size,
        };

        let mut new_files = Vec::<FileReference>::new();
        new_files.push(file);

        let new_resource = Resource {
            id: r.id,
            user_id: r.user_id,
            group_id: r.group_id,
            title: r.title,
            description: r.description,
            subject: r.subject,
            tags: r.tags,
            files: Option::from(new_files),
            last_edited_at: r.last_edited_at,
        };

        let _ = database
            .collection::<Resource>("resources")
            .replace_one(query, new_resource, None)
            .await
            .expect("Error updating document");

    }
}
