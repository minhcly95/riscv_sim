use bytesize::ByteSize;
use clap::{Parser, ValueHint};
use clap_num::maybe_hex;
use std::path::PathBuf;

/// A simple RISC-V simulation
#[derive(Parser, Debug)]
#[command(version)]
pub struct Config {
    /// Binary file to load into RAM
    #[arg(required = true, value_hint = ValueHint::FilePath)]
    pub binary: Option<PathBuf>,

    /// Size of the RAM
    #[arg(short = 's', long, default_value_t = ByteSize::mib(128))]
    pub size: ByteSize,

    /// Base address of the RAM
    #[arg(short = 'b', long, default_value_t = 0, value_parser = maybe_hex::<u32>)]
    pub base: u32,

    /// Device tree blob
    #[arg(long, value_hint = ValueHint::FilePath)]
    pub dtb: Option<PathBuf>,

    /// Kernel (load at 0x00400000)
    #[arg(short = 'k', long, value_hint = ValueHint::FilePath)]
    pub kernel: Option<PathBuf>,

    /// Print extra information
    #[arg(short = 'v', long)]
    pub verbose: bool,
}

pub enum ConfigError {
    InvalidBinary(PathBuf),
    InvalidDtb(PathBuf),
    InvalidKernel(PathBuf),
}

impl Config {
    pub fn new() -> Config {
        Config {
            binary: None,
            size: ByteSize::mib(128),
            base: 0,
            dtb: None,
            kernel: None,
            verbose: true,
        }
    }

    pub fn validate(self) -> Result<Config, ConfigError> {
        if let Some(path) = &self.binary {
            if !path.is_file() {
                return Err(ConfigError::InvalidBinary(self.binary.unwrap()));
            }
        }
        if let Some(path) = &self.dtb {
            if !path.is_file() {
                return Err(ConfigError::InvalidDtb(self.dtb.unwrap()));
            }
        }
        if let Some(path) = &self.kernel {
            if !path.is_file() {
                return Err(ConfigError::InvalidKernel(self.dtb.unwrap()));
            }
        }
        Ok(self)
    }
}
