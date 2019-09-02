use crate::common::*;

#[derive(StructOpt)]
pub(crate) struct Opt {
  report: Option<PathBuf>,
}

impl Opt {
  pub(crate) fn run(self) -> Result<(), Error> {
    if let Some(report) = self.report {
      Self::report(&report)
    } else {
      Implementation::run_all()
    }
  }

  fn report(report: &Path) -> Result<(), Error> {
    info!("generating report in {}", report.display());
    let vars = env::vars_os()
      .map(|(key, value)| {
        (
          key.to_string_lossy().into_owned(),
          value.to_string_lossy().into_owned(),
        )
      })
      .filter(|(key, _)| key.starts_with("DOTENV_"))
      .collect::<BTreeMap<String, String>>();

    let json = serde_json::to_string(&vars).map_err(Error::ReportSerialize)?;

    fs::write(report, json).map_err(|io_error| Error::Io {
      io_error,
      path: report.to_owned(),
    })?;

    Ok(())
  }
}
