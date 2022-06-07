use log::info;

use jolly_dumper::JollyDumper;

fn main() {
  JollyDumper::init();
  info!("Running jolly dumper");
}
