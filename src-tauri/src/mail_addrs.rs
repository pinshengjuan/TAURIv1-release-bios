pub mod get_address_from_config{
  use regex::Regex;
  use std::fs::read_to_string;

  pub struct GetAddress {
    config_file: String,
  }

  impl GetAddress {

    pub fn new(config_file: Option<String>) -> Self {
      let config_file = config_file.unwrap_or_else(|| "".to_string());
      GetAddress { config_file }
    }

    fn search_address_from_config(&self, reg: Regex, replace: &str) -> String {
      for content in read_to_string(&self.config_file).unwrap().lines() {
          if let Some(line) = content.lines().find(|line| reg.is_match(line)) {
              return line.replace(replace, "").trim().to_string()
          }
      }

      "".to_string()
    }
    pub fn qt_leader_email(&self) -> String {
        let qt = Regex::new(r"(?m)^QT:").unwrap();
        self.search_address_from_config(qt, "QT:")
    }
    pub fn bios_qt_leader_email(&self) -> String {
        let bios_qt = Regex::new(r"(?m)^BIOS QT:").unwrap();
        self.search_address_from_config(bios_qt, "BIOS QT:")
    }
  }

}