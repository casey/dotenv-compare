mod common;
mod dir;
mod error;
mod implementation;
mod language;
mod opt;
mod path;
mod report;
mod testing_procedure_specification_report;

use crate::common::*;

fn main() -> Result<(), Error> {
  pretty_env_logger::init();
  Opt::from_args().run()
}
