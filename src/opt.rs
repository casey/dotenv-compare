use crate::common::*;

#[derive(StructOpt)]
pub(crate) enum Opt {
  #[structopt(name = "generate", help = "update all reports")]
  Generate,
  #[structopt(name = "save", help = "save a generated report")]
  Save {
    #[structopt(long = "language")]
    language: Language,
    #[structopt(long = "version")]
    version: Version,
    #[structopt(long = "destination")]
    destination: PathBuf,
  },
}

impl Opt {
  pub(crate) fn run(self) -> Result<(), Error> {
    match self {
      Opt::Generate => Self::generate(),
      Opt::Save {
        language,
        version,
        destination,
      } => Self::save(language, version, &destination),
    }
  }

  fn generate() -> Result<(), Error> {
    for entry in dir::entries(Implementation::directory())? {
      let implementation: Implementation = entry.file_name().to_string_lossy().parse()?;

      implementation.generate_report()?;
    }

    let reports = dir::entries(Report::directory())?
      .iter()
      .map(|entry| Report::load(&entry.path()))
      .collect::<Result<Vec<Report>, Error>>()?;

    let tpsr = TestingProcedureSpecificationReport::from_reports(reports);

    let csv = tpsr.csv();

    let destination = Path::new("summary.csv");

    fs::write(destination, csv).map_err(|io_error| Error::Io {
      io_error,
      path: destination.to_owned(),
    })?;

    Ok(())
  }

  fn save(language: Language, version: Version, destination: &Path) -> Result<(), Error> {
    let json = Report::from_env(language, version.clone()).json()?;
    fs::write(&destination, json).map_err(|io_error| Error::Io {
      io_error,
      path: destination.to_owned(),
    })
  }
}
