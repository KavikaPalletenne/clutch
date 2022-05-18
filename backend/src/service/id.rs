use snowflake::SnowflakeIdGenerator;

pub fn generate_snowflake() -> i64 {
    let mut id_generator = SnowflakeIdGenerator::new(1, 1);
    id_generator.real_time_generate()
}
