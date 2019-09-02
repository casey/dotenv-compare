use crate::common::*;

pub(crate) struct TestingProcedureSpecificationReport {
  reports: Vec<Report>,
}

impl TestingProcedureSpecificationReport {
  pub(crate) fn from_reports(reports: Vec<Report>) -> TestingProcedureSpecificationReport {
    TestingProcedureSpecificationReport { reports }
  }

  pub(crate) fn csv(&self) -> String {
    let mut bytes = Vec::new();

    let tests = self.reports[0]
      .results
      .keys()
      .cloned()
      .collect::<Vec<String>>();

    {
      let mut writer = csv::Writer::from_writer(Cursor::new(&mut bytes));

      let mut headers = Vec::new();

      headers.push("test".to_string());

      for report in &self.reports {
        headers.push(format!(
          "{} {}",
          report.implementation.language, report.implementation.version,
        ));
      }

      writer.write_record(headers).unwrap();

      for test in tests {
        let mut row = Vec::new();

        let name = test.trim_start_matches("DOTENV_COMPARE_").to_lowercase();

        row.push(&name);

        for report in &self.reports {
          row.push(&report.results.get(&test).unwrap());
        }

        writer.write_record(row).unwrap();
      }

      writer.flush().unwrap();
    }

    String::from_utf8(bytes).unwrap()
  }
}
