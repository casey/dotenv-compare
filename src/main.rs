mod common;
mod error;
mod implementation;
mod opt;

use crate::common::*;

fn main() -> Result<(), Error> {
  pretty_env_logger::init();
  Opt::from_args().run()
}
