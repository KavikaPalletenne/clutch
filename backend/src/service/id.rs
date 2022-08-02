use nanoid::nanoid;
use snowflake::SnowflakeIdGenerator;

pub fn generate_snowflake() -> i64 {
    let mut id_generator = SnowflakeIdGenerator::new(1, 1);
    id_generator.real_time_generate()
}

static ALPHANUMERIC_ALPHABET: [char; 62] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z',
];

pub fn generate_alphanumeric_nanoid(size: usize) -> String {
    nanoid!(size, &ALPHANUMERIC_ALPHABET)
}
