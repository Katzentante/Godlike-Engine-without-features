use std::env::{self, VarError};

fn main() -> Result<(), VarError> {
    // This tells cargo to rerun this script if something in /res/ changes.
    println!("cargo:rerun-if-changed=Config.toml");

    let out_dir = env::var("OUT_DIR")?;
    Ok(())
}
