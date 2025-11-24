use anyhow::Result;
use std::path::PathBuf;

pub fn init() -> Result<()> {
    let log_dir = get_log_dir()?;
    std::fs::create_dir_all(&log_dir)?;
    
    let log_file = log_dir.join("nexus.log");
    let file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)?;
    
    tracing_subscriber::fmt()
        .with_writer(file)
        .with_ansi(false)
        .init();
    
    Ok(())
}

fn get_log_dir() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
    
    Ok(config_dir.join("nexus").join("logs"))
}

