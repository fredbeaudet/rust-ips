#![allow(unused)]

use std::env;
use std::process;
use clap::{Parser, Subcommand};

mod commands;
mod parser;
mod apply;

fn main() {
    match commands::Args::parse().command {
        commands::Commands::Apply { patch_file, file_to_patch, output_file } => {
            apply::apply(&patch_file, &file_to_patch, &output_file);
        },
        commands::Commands::Create { unpatched_file, patched_file, output_file } => {
            println!("todo: implement function to create IPS patch")
        }
    }
}
