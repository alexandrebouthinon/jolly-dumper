use clap::Parser;
use log::info;

use jolly_dumper::Cli;

fn main() {
  let args = Cli::parse();
  env_logger::Builder::new()
    .filter_level(args.verbose.log_level_filter())
    .init();
  info!("Running jolly dumper");
}
