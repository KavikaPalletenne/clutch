use std::env;
use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::Region;

pub fn init_bucket() -> Bucket {
    let bucket_name = "excl";
    let access_key = env::var("S3_ACCESS_KEY").expect("Error getting S3 CREDENTIALS").to_string();
    let secret_key = env::var("S3_SECRET_KEY").expect("Error getting S3 CREDENTIALS").to_string();
    // let security_token = env::var("S3_SECURITY_TOKEN").expect("Error getting S3 CREDENTIALS").to_string();
    // let session_token = env::var("S3_SESSION_TOKEN").expect("Error getting S3 CREDENTIALS").to_string();

    // let credentials = Credentials::new(
    //     Option::from(access_key.as_str()),
    //     Option::from(secret_key.as_str()),
    //     None,
    //     None,
    //     None
    // ).unwrap();

    let credentials = Credentials::new(
        Option::from("excl-api"),
        Option::from("kavikabhasura"),
        None,
        None,
        None
    ).unwrap();

    let region_name = "dev".to_string();
    let endpoint = "http://api.examclutch.com:8080".to_string();
    let region = Region::Custom { region: region_name, endpoint };

    println!("Bucket initialised");
    //Bucket::new_with_path_style(bucket_name, region, credentials).unwrap()
    Bucket::new(bucket_name, region, credentials).unwrap().with_path_style()
}
