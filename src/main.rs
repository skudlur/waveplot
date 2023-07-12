/*  waveplot  */

use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Takes the VCD file
    #[arg(short, long, value_name = "FILE")]
    vcd: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    if let Some(vcd_file) = cli.vcd.as_deref() {
        println!("Value change dump file: {}", vcd_file.display());
    }
}
