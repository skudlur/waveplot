/*  waveplot  */

#![allow(non_snake_case)]
use clap::Parser;
use std::path::PathBuf;

mod VCD;
use env_logger::{Builder, Target};
use log::error;
use log::LevelFilter;
use VCD::VCDParser;

#[derive(Parser)]
#[command(name = "waveplot")]
#[command(version = "0.1")]
#[command(about = "TUI waveplot generator.", long_about = None)]
struct Cli {
    /// Takes the VCD file
    #[arg(short, long, value_name = "FILE")]
    vcd: Option<PathBuf>,
    #[arg(long)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    let mut log_builder = Builder::new();
    log_builder.target(Target::Stdout);
    log_builder.filter_level(LevelFilter::Info);
    if cli.verbose == true {
        log_builder.filter_module("waveplot::VCD::VCDParser", LevelFilter::Debug);
    } else {
        log_builder.filter_module("waveplot::VCD::VCDParser", LevelFilter::Warn);
    }
    log_builder.init();

    if let Some(vcd_file) = cli.vcd.as_deref() {
        println!("Value change dump file: {}", vcd_file.display());
        let (guide_map, parsed_vcd) = VCDParser::vcd_parser_wrapper(vcd_file.to_str().unwrap());
        //TUI takes these two above and renders it
    } else {
        error!("Please provide to VCD file using --vcd option.");
    }
}
