fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/authservice.proto")?;
    tonic_build::compile_protos("proto/user/userservice.proto")?;
    Ok(())
}
