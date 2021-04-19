//! CLI parameters structs used to wrap static str references.

/// Holds static str references for a CLI option.
pub struct CliOpt {
  /// Option's short name represented with a letter.
  ///
  /// If there is no short name, it must be set to `""`.
  pub short_name: &'static str,
  /// Option's long name represented with a word or `-`-separated words.
  pub long_name: &'static str,
  /// Option's description.
  pub description: &'static str,
  /// Option's placeholder to display within help message.
  pub placeholder: &'static str,
  /// Option's associated environment variable name.
  ///
  /// If there is no associated environment variable, it must be set to `""`.
  pub evar_name: &'static str,
}

impl CliOpt {
  pub const fn new(
    short_name: &'static str,
    long_name: &'static str,
    description: &'static str,
    placeholder: &'static str,
    evar_name: &'static str,
  ) -> CliOpt {
    CliOpt {
      short_name,
      long_name,
      description,
      placeholder,
      evar_name,
    }
  }
}

pub struct CliFlag {
  /// Flag's short name represented with a letter.
  ///
  /// If there is no short name, it must be set to `""`.
  pub short_name: &'static str,
  /// Flag's long name represented with a word or `-`-separated words.
  pub long_name: &'static str,
  /// Option's description.
  pub description: &'static str,
}

impl CliFlag {
  pub const fn new(
    short_name: &'static str,
    long_name: &'static str,
    description: &'static str,
  ) -> CliFlag {
    CliFlag {
      short_name,
      long_name,
      description,
    }
  }
}
