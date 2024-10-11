//use std::env;
//use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::env::var("OUT_DIR")?;
    println!("OUT_DIR: {}", out_dir);

    // Path to the proto-definitions
    //let proto_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("proto-definitions");

    // Run a command to ensure the proto-definitions directory exists
    Command::new("git")
        .args(&["clone", "https://github.com/bennekrouf/proto-definitions"])
        .status()
        .expect("Failed to clone proto-definitions");

    tonic_build::configure().out_dir(&out_dir).compile(
        &["proto-definitions/proto/messenger.proto"],
        &["proto-definitions/proto"],
    )?;
    Ok(())
}
