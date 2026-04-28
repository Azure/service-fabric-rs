fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::compile_protos("proto/helloworld.proto")?;
    tonic_prost_build::compile_protos("proto/initdata.proto")?;
    tonic_prost_build::compile_protos("proto/control.proto")?;
    Ok(())
}
