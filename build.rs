use std::{env, path::PathBuf};
use tonic_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(&["./proto/authuser.proto"], &["./proto/"])?;
    // tonic_build::compile_protos("./proto/auth.proto")?;

    Ok(())
}
