use crate::common::*;

#[derive(Serialize, Deserialize)]
pub(crate) struct Implementation {
  pub(crate) language: Language,
  pub(crate) version: Version,
}

impl Implementation {
  pub(crate) fn directory() -> &'static Path {
    Path::new("implementation")
  }

  pub(crate) fn generate_report(&self) -> Result<(), Error> {
    let executable = path::canonicalize(env::args().next().unwrap())?;

    let destination = path::canonicalize(Report::directory())?
      .join(self.slug())
      .with_extension("json");

    let directory = Self::directory().join(self.slug());

    let args_owned: Vec<OsString> = vec![
      executable.into(),
      "save".into(),
      "--language".into(),
      self.language.to_string().into(),
      "--version".into(),
      self.version.to_string().into(),
      "--destination".into(),
      destination.into(),
    ];

    let args: Vec<&OsStr> = args_owned.iter().map(|arg| arg.as_ref()).collect();

    self.language.generate_report(&directory, &args)
  }

  fn slug(&self) -> String {
    format!("{}-{}", self.language, self.version)
  }
}

impl FromStr for Implementation {
  type Err = Error;

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    let parts = text.split('-').collect::<Vec<&str>>();

    if parts.len() != 2 {
      return Err(Error::ImplementationFormat {
        text: text.to_owned(),
      });
    }

    Ok(Implementation {
      language: parts[0].parse()?,
      version: parts[1]
        .parse()
        .map_err(|semver_error| Error::Version { semver_error })?,
    })
  }
}
