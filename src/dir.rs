use crate::common::*;

  pub(crate) fn entries(path: &Path) -> Result<Vec<DirEntry>, Error> {
    fs::read_dir(path)
      .map_err(|io_error| Error::Io {
        path: path.to_owned(),
        io_error,
      })?
      .map(|entry| {
        entry.map_err(|io_error| Error::Io {
          path: path.to_owned(),
          io_error,
        })
      })
      .collect()
  }

