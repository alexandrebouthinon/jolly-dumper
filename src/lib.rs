use clap::Parser;
use clap_verbosity_flag::Verbosity;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct JollyDumper {
  #[clap(flatten)]
  pub verbose: Verbosity,
}

impl JollyDumper {
  pub fn init() -> Self {
    let cli = Self::parse();
    env_logger::Builder::new()
      .filter_level(cli.verbose.log_level_filter())
      .init();

    cli
  }
}
