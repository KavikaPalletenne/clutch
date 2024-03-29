use s3::creds::Credentials;
use s3::{Bucket, Region};
use std::env;

pub fn init_bucket() -> Bucket {
    let bucket_name = "excl-cdn";
    let access_key = env::var("S3_ACCESS_KEY")
        .expect("Error getting S3 CREDENTIALS")
        .to_string();
    let secret_key = env::var("S3_SECRET_KEY")
        .expect("Error getting S3 CREDENTIALS")
        .to_string();
    let endpoint = env::var("S3_ENDPOINT")
        .expect("Error getting S3 ENDPOINT")
        .to_string();
    let region_name = env::var("S3_REGION_NAME")
        .expect("Error getting S3 REGION NAME")
        .to_string();

    let credentials = Credentials::new(
        Option::from(access_key.as_str()),
        Option::from(secret_key.as_str()),
        None,
        None,
        None,
    )
    .unwrap();

    let region = Region::Custom {
        region: region_name,
        endpoint,
    };

    println!("Bucket initialised");
    //Bucket::new_with_path_style(bucket_name, region, credentials).unwrap()
    Bucket::new(bucket_name, region, credentials).unwrap()
}
