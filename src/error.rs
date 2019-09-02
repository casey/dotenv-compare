use crate::common::*;

#[derive(Debug)]
pub(crate) enum Error {
  Canonicalize {
    path: PathBuf,
    io_error: io::Error,
  },
  Io {
    io_error: io::Error,
    path: PathBuf,
  },
  ReportSerialize {
    serialize_error: serde_json::Error,
  },
  ReportDeserialize {
    deserialize_error: serde_json::Error,
  },
  LanguageUnknown {
    text: String,
  },
  ImplementationFormat {
    text: String,
  },
  ImplementationInvoke {
    directory: PathBuf,
    io_error: io::Error,
  },
  ImplementationExitStatus {
    directory: PathBuf,
    exit_status: ExitStatus,
  },
  Version {
    semver_error: semver::SemVerError,
  },
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "error: {:?}", self)
  }
}
