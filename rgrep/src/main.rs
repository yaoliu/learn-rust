use anyhow::Result;
use rgrep::*;
use clap::Parser;


fn main() -> Result<()> {
    let mut config: GrepConfig = GrepConfig::parse();
    config.match_with_default()?;
    Ok(())
}