use super::Configuration;
use dprint_core::configuration::*;

/// Resolves configuration from a collection of key value strings.
///
/// # Example
///
/// ```
/// use dprint_core::configuration::ConfigKeyMap;
/// use dprint_core::configuration::resolve_global_config;
/// use dprint_plugin_alejandra::configuration::resolve_config;
///
/// let mut config_map = ConfigKeyMap::new(); // get a collection of key value pairs from somewhere
/// let global_config_result = resolve_global_config(&mut config_map);
///
/// // check global_config_result.diagnostics here...
///
/// let config_map = ConfigKeyMap::new(); // get a collection of k/v pairs from somewhere
/// let config_result = resolve_config(
///     config_map,
///     &global_config_result.config
/// );
///
/// // check config_result.diagnostics here and use config_result.config
/// ```
pub fn resolve_config(
  config: ConfigKeyMap,
  global_config: &GlobalConfiguration,
) -> ResolveConfigurationResult<Configuration> {
  let mut diagnostics = Vec::new();
  let mut config = config;

  // the defaults are Alejandra's rather than dprint's recommended ones. Alejandra
  // only indents with two spaces, four spaces or tabs, so a global indent width
  // it doesn't support falls back to its default instead of being an error
  let global_indent_width = global_config
    .indent_width
    .filter(|width| is_supported_indent_width(*width));
  let resolved_config = Configuration {
    use_tabs: get_value(
      &mut config,
      "useTabs",
      global_config.use_tabs.unwrap_or(false),
      &mut diagnostics,
    ),
    indent_width: get_value(
      &mut config,
      "indentWidth",
      global_indent_width.unwrap_or(2),
      &mut diagnostics,
    ),
    new_line_kind: get_value(
      &mut config,
      "newLineKind",
      global_config.new_line_kind.unwrap_or(NewLineKind::LineFeed),
      &mut diagnostics,
    ),
    space_around_brackets: get_value(&mut config, "spaceAroundBrackets", false, &mut diagnostics),
  };

  if !is_supported_indent_width(resolved_config.indent_width) {
    diagnostics.push(ConfigurationDiagnostic {
      property_name: "indentWidth".to_string(),
      message: format!(
        "Alejandra only supports an indent width of 2 or 4, but found {}.",
        resolved_config.indent_width
      ),
    });
  }

  diagnostics.extend(get_unknown_property_diagnostics(config));

  ResolveConfigurationResult {
    config: resolved_config,
    diagnostics,
  }
}

fn is_supported_indent_width(width: u8) -> bool {
  width == 2 || width == 4
}
