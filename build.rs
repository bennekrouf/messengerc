fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::env::var("OUT_DIR")?;
    println!("OUT_DIR: {}", out_dir);

    // Path to the proto-definitions
    //let proto_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("proto-definitions");

    tonic_build::configure().out_dir(&out_dir).compile(
        &["../proto-definitions/proto/messenger.proto"],
        &["../proto-definitions/proto"],
    )?;
    Ok(())
}
