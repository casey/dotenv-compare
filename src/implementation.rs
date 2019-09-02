use crate::common::*;

pub(crate) enum Implementation {
  Rust,
  Ruby,
}

impl Implementation {
  pub(crate) fn run_all() -> Result<(), Error> {
    let binary = fs::canonicalize(env::args().next().unwrap()).map_err(Error::Canonicalize)?;

    let implementations = Path::new("implementation");

    for entry in fs::read_dir(implementations).map_err(|io_error| Error::Io {
      path: implementations.to_owned(),
      io_error,
    })? {
      let entry = entry.map_err(|io_error| Error::Io {
        path: implementations.to_owned(),
        io_error,
      })?;

      Self::run(&binary, entry)?;
    }
    Ok(())
  }

  pub(crate) fn run(binary: &Path, entry: DirEntry) -> Result<(), Error> {
    let file_name = entry.file_name().to_str().unwrap().to_owned();

    let parts = file_name.split('-').collect::<Vec<&str>>();

    assert_eq!(parts.len(), 2);

    let name = parts[0];

    let implementation = Self::from_name(name);

    implementation.create_report(binary, entry)?;

    Ok(())
  }

  fn from_name(name: &str) -> Implementation {
    match name {
      "rust" => Implementation::Rust,
      "ruby" => Implementation::Ruby,
      _ => panic!("unexpected implementation name: {}", name),
    }
  }

  fn create_report(self, binary: &Path, entry: DirEntry) -> Result<(), Error> {
    info!("creating report for {}", entry.path().display());

    let report = Path::new("./report")
      .canonicalize()
      .map_err(Error::Canonicalize)?
      .join(entry.file_name())
      .with_extension("json");

    match self {
      Self::Rust => {
        let manifest = entry.path().join("Cargo.toml");

        Command::new("cargo")
          .arg("run")
          .arg("--manifest-path")
          .arg(manifest)
          .arg(binary)
          .arg(report)
          .status()
          .unwrap();

        Ok(())
      }

      Self::Ruby => {
        Command::new("bundle")
          .arg("exec")
          .arg("./main")
          .arg(binary)
          .arg(report)
          .current_dir(entry.path())
          .status()
          .unwrap();

        Ok(())
      }
    }
  }
}
