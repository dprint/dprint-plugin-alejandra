use std::path::Path;

use alejandra::config::Config;
use alejandra::config::Indentation;
use alejandra::format::Status;
use dprint_core::configuration::resolve_new_line_kind;

use crate::configuration::Configuration;

/// Error that occurred while formatting Nix text.
#[derive(Debug, thiserror::Error)]
pub enum FormatError {
  /// The text could not be parsed by Alejandra.
  #[error("{0}")]
  Parse(String),
}

pub fn format_text(file_path: &Path, input_text: &str, config: &Configuration) -> Result<Option<String>, FormatError> {
  // Alejandra writes its own newlines as line feeds, but keeps the carriage
  // returns inside strings, so give it line feeds only and apply the newline
  // kind afterwards.
  let new_line = resolve_new_line_kind(input_text, config.new_line_kind);
  let (status, formatted) = alejandra::format::in_memory(
    file_path.to_string_lossy().into_owned(),
    input_text.replace("\r\n", "\n"),
    build_config(config),
  );
  if let Status::Error(message) = status {
    return Err(FormatError::Parse(message));
  }

  let formatted = if new_line == "\n" {
    formatted
  } else {
    formatted.replace('\n', new_line)
  };
  if formatted == input_text {
    Ok(None)
  } else {
    Ok(Some(formatted))
  }
}

fn build_config(config: &Configuration) -> Config {
  Config {
    indentation: if config.use_tabs {
      Indentation::Tabs
    } else if config.indent_width == 4 {
      Indentation::FourSpaces
    } else {
      Indentation::TwoSpaces
    },
    space_around_brackets: config.space_around_brackets,
  }
}
