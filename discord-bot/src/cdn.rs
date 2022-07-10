use s3::Bucket;

pub fn get_download_file_url(resource_id: String, file_name: String, bucket: &Bucket) -> String {
    bucket
        .presign_get(format!("/{}/{}", resource_id, file_name), 3600, None)
        .unwrap() // 1 hour expiry
}
