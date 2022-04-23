use s3::{Bucket, Region};
use s3::creds::Credentials;

pub fn create_s3_bucket(
    bucket_name: String,
    access_key: String,
    secret_key: String,
    region_name: String,
    endpoint: String,
) -> Bucket {
    let credentials = Credentials::new(
    Option::from(&*access_key),
    Option::from(&*secret_key),
None,
None,
    None,
    ).unwrap();

    let region = Region::Custom { region: region_name, endpoint };
    Bucket::new(&*bucket_name, region, credentials).unwrap().with_path_style()
}
