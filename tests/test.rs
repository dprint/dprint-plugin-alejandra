extern crate dprint_development;
extern crate dprint_plugin_alejandra;

use std::path::PathBuf;
use std::sync::Arc;

use dprint_core::configuration::*;
use dprint_development::*;
use dprint_plugin_alejandra::configuration::Configuration;
use dprint_plugin_alejandra::configuration::resolve_config;
use dprint_plugin_alejandra::*;

#[test]
fn test_specs() {
  let global_config = GlobalConfiguration::default();

  run_specs(
    &PathBuf::from("./tests/specs"),
    &ParseSpecOptions {
      default_file_name: "file.nix",
    },
    &RunSpecsOptions {
      fix_failures: false,
      format_twice: true,
    },
    {
      let global_config = global_config.clone();
      Arc::new(move |file_path, file_text, _range, spec_config| {
        let spec_config: ConfigKeyMap = serde_json::from_value(spec_config.clone().into()).unwrap();
        let config_result = resolve_config(spec_config, &global_config);
        ensure_no_diagnostics(&config_result.diagnostics);

        format_text(file_path, file_text, &config_result.config).map_err(|err| err.into())
      })
    },
    Arc::new(move |_file_path, _file_text, _spec_config| panic!("Plugin does not support dprint-core tracing.")),
  )
}

#[test]
fn should_fail_on_parse_error() {
  let err = format_text(&PathBuf::from("./file.nix"), "{ a = ", &default_config()).unwrap_err();
  assert_eq!(err.to_string(), "unexpected end of file");
}

#[test]
fn should_report_unsupported_indent_width() {
  let mut config = ConfigKeyMap::new();
  config.insert("indentWidth".to_string(), ConfigKeyValue::Number(3));
  let result = resolve_config(config, &GlobalConfiguration::default());
  let property_names = result
    .diagnostics
    .iter()
    .map(|d| d.property_name.as_str())
    .collect::<Vec<_>>();
  assert_eq!(property_names, ["indentWidth"]);
}

#[test]
fn should_ignore_unsupported_global_indent_width() {
  let global_config = GlobalConfiguration {
    indent_width: Some(3),
    ..Default::default()
  };
  let result = resolve_config(ConfigKeyMap::new(), &global_config);
  assert!(result.diagnostics.is_empty());
  assert_eq!(result.config.indent_width, 2);
}

#[test]
fn should_use_global_indent_width() {
  let global_config = GlobalConfiguration {
    indent_width: Some(4),
    ..Default::default()
  };
  let result = resolve_config(ConfigKeyMap::new(), &global_config);
  assert_eq!(result.config.indent_width, 4);
}

#[test]
fn should_use_lf_for_crlf_input() {
  let result = format_text(
    &PathBuf::from("./file.nix"),
    "{\r\na = ''\r\n  x\r\n'';\r\n}\r\n",
    &default_config(),
  )
  .unwrap();
  assert_eq!(result.unwrap(), "{\n  a = ''\n    x\n  '';\n}\n");
}

#[test]
fn should_keep_crlf_with_auto() {
  let mut config = default_config();
  config.new_line_kind = NewLineKind::Auto;
  let result = format_text(
    &PathBuf::from("./file.nix"),
    "{\r\na = ''\r\n  x\r\n'';\r\n}\r\n",
    &config,
  )
  .unwrap();
  assert_eq!(result.unwrap(), "{\r\n  a = ''\r\n    x\r\n  '';\r\n}\r\n");
}

#[test]
fn should_return_none_for_formatted_crlf_text() {
  let mut config = default_config();
  config.new_line_kind = NewLineKind::CarriageReturnLineFeed;
  let result = format_text(&PathBuf::from("./file.nix"), "{\r\n  a = 1;\r\n}\r\n", &config).unwrap();
  assert!(result.is_none());
}

#[test]
fn should_have_schema_defaults_matching_resolved_config() {
  let schema: serde_json::Value = serde_json::from_str(include_str!("../deployment/schema.json")).unwrap();
  let properties = schema["properties"].as_object().unwrap();
  let config = serde_json::to_value(default_config()).unwrap();
  let config = config.as_object().unwrap();
  assert_eq!(properties.len(), config.len());
  for (key, value) in config {
    assert_eq!(&properties[key]["default"], value, "default of {key}");
  }
}

fn default_config() -> Configuration {
  resolve_config(Default::default(), &GlobalConfiguration::default()).config
}
