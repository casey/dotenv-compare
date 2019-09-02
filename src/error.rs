use crate::common::*;

#[derive(Debug)]
pub(crate) enum Error {
  Canonicalize(io::Error),
  Io { io_error: io::Error, path: PathBuf },
  ReportSerialize(serde_json::Error),
}
