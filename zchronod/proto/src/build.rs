
use std::io::Result;

// used to build proto
pub fn set() -> Result<()> {
    tonic_build::configure()
        .out_dir("proto/src")
        .compile(&["proto/src/zchronod.proto"], &["proto/"])?;
    Ok(())
}
