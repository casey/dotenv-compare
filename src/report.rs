use crate::common::*;

#[derive(Deserialize, Serialize)]
pub(crate) struct Report {
  pub(crate) implementation: Implementation,
  pub(crate) results: BTreeMap<String, String>,
}

impl Report {
  pub(crate) fn directory() -> &'static Path {
    Path::new("report")
  }

  pub(crate) fn from_env(language: Language, version: Version) -> Report {
    let results = env::vars_os()
      .map(|(key, value)| {
        (
          key.to_string_lossy().into_owned(),
          value.to_string_lossy().into_owned(),
        )
      })
      .filter(|(key, _)| key.starts_with("DOTENV_COMPARE_"))
      .collect();

    Report {
      implementation: Implementation { language, version },
      results,
    }
  }

  pub(crate) fn json(&self) -> Result<String, Error> {
    serde_json::to_string_pretty(self)
      .map_err(|serialize_error| Error::ReportSerialize { serialize_error })
  }

  pub(crate) fn load(path: &Path) -> Result<Report, Error> {
    let json = fs::read_to_string(path).map_err(|io_error| Error::Io {
      io_error,
      path: path.to_owned(),
    })?;

    Ok(
      serde_json::from_str(&json)
        .map_err(|deserialize_error| Error::ReportDeserialize { deserialize_error })?,
    )
  }
}
