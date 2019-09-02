use crate::common::*;

pub(crate) fn canonicalize(path: impl AsRef<Path>) -> Result<PathBuf, Error> {
  fs::canonicalize(path.as_ref()).map_err(|io_error| Error::Canonicalize {
    io_error,
    path: path.as_ref().to_owned(),
  })
}
