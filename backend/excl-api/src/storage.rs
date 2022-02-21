use std::env;
use s3::bucket::Bucket;
use s3::creds::Credentials;

pub fn init_bucket() -> Bucket {
    let bucket_name = "excl";
    let region = "sgp1".parse().unwrap();
    let access_key = env::var("S3_ACCESS_KEY").expect("Error getting S3 CREDENTIALS").to_string();
    let secret_key = env::var("S3_SECRET_KEY").expect("Error getting S3 CREDENTIALS").to_string();
    // let security_token = env::var("S3_SECURITY_TOKEN").expect("Error getting S3 CREDENTIALS").to_string();
    // let session_token = env::var("S3_SESSION_TOKEN").expect("Error getting S3 CREDENTIALS").to_string();

    let credentials = Credentials::new(
        Some(access_key.as_str()),
        Some(secret_key.as_str()),
        None,
        None,
        None
    ).unwrap();

    Bucket::new(bucket_name, region, credentials).unwrap()
}
