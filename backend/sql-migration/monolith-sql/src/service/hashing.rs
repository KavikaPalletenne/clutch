use argon2::{Config, ThreadMode, Variant, Version};
use std::env;

pub struct ArgonSetup<'a> {
    pub salt: String,
    pub config: Config<'a>,
}

pub fn setup_argon2() -> ArgonSetup<'static> {
    let salt = env::var("ARGON_SALT").unwrap();
    let config = Config {
        variant: Variant::Argon2id,
        version: Version::Version13,
        mem_cost: 65536,
        time_cost: 10,
        lanes: 4,
        thread_mode: ThreadMode::Parallel,
        secret: &[],
        ad: &[],
        hash_length: 32,
    };

    ArgonSetup { salt, config }
}

/// Hashes the given string using the given config and returns the hash.
pub fn hash(string: String) -> String {
    let config = setup_argon2(); // TODO: See if setting this up at start and using as web::Data<> is faster
    argon2::hash_encoded(string.as_bytes(), config.salt.as_bytes(), &config.config).unwrap()
}

/// Checks whether the password matches the hash.
/// Returns true if it is a match, otherwise false.
pub fn verify(password: String, hash: String, config: &ArgonSetup) -> bool {
    argon2::verify_encoded(hash.as_str(), password.as_bytes()).unwrap()
}
