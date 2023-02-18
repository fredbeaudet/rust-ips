use std::env;
use std::process;
use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Commands {
    Apply { 
        patch_file: std::path::PathBuf, 
        file_to_patch: std::path::PathBuf,
        output_file: std::path::PathBuf,
    },
    Create {
        unpatched_file: std::path::PathBuf,
        patched_file: std::path::PathBuf,
        output_file: std::path::PathBuf,
    },
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}
