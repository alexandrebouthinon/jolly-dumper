use clap::Parser;
use clap_verbosity_flag::Verbosity;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
  #[clap(flatten)]
  pub verbose: Verbosity,
}
