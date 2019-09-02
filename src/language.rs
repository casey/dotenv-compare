use crate::common::*;

#[derive(Serialize, Deserialize, Copy, Clone)]
pub(crate) enum Language {
  #[serde(rename = "rust")]
  Rust,
  #[serde(rename = "ruby")]
  Ruby,
}

impl Language {
  const RUST: &'static str = "rust";
  const RUBY: &'static str = "ruby";

  pub(crate) fn generate_report(&self, directory: &Path, args: &[&OsStr]) -> Result<(), Error> {
    info!("creating report for `{}`", directory.display());
    info!("arguments: {:?}", args);

    let mut command = match self {
      Self::Rust => {
        let manifest = directory.join("Cargo.toml");

        let mut command = Command::new("cargo");

        command.arg("run").arg("--manifest-path").arg(manifest);

        command
      }

      Self::Ruby => {
        let mut command = Command::new("bundle");

        command.arg("exec").arg("./main").current_dir(directory);

        command
      }
    };

    command.args(args);

    let exit_status = command
      .status()
      .map_err(|io_error| Error::ImplementationInvoke {
        io_error,
        directory: directory.to_owned(),
      })?;

    if !exit_status.success() {
      return Err(Error::ImplementationExitStatus {
        exit_status,
        directory: directory.to_owned(),
      });
    }

    Ok(())
  }
}

impl Display for Language {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::Rust => write!(f, "{}", Self::RUST),
      Self::Ruby => write!(f, "{}", Self::RUBY),
    }
  }
}

impl FromStr for Language {
  type Err = Error;

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    match text {
      Self::RUST => Ok(Self::Rust),
      Self::RUBY => Ok(Self::Ruby),
      _ => Err(Error::LanguageUnknown {
        text: text.to_owned(),
      }),
    }
  }
}
