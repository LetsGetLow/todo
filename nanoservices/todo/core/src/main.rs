mod api;
mod enums;
mod structs;

use clap::Parser;
use shared::errors::NanoServiceError;

// Simple program for booking
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    title: String,
    #[arg(short, long)]
    status: String,
}

fn main() -> Result<(), NanoServiceError> {
    Ok(())
}
