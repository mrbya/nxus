use std::fs;
use std::path::Path;

use indexmap::IndexMap;
use toml_edit::{DocumentMut, RawString};

use crate::{CommandConfig, CoreResult};

/// Display metadata for a configured project command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandInfo {
    /// Configured project command name.
    pub name: String,
    /// Optional single-line description extracted from the adjacent TOML comment.
    pub description: Option<String>,
}

/// Reads configured project commands and their optional adjacent comment descriptions.
///
/// # Errors
/// Returns an I/O error when `nxus.toml` cannot be read.
pub fn command_info(
    project_dir: &Path,
    commands: &IndexMap<String, CommandConfig>,
) -> CoreResult<Vec<CommandInfo>> {
    let source = fs::read_to_string(project_dir.join("nxus.toml"))?;
    let descriptions = parse_command_descriptions(&source).unwrap_or_default();

    Ok(commands
        .keys()
        .map(|name| CommandInfo {
            name: name.clone(),
            description: descriptions.get(name).cloned().flatten(),
        })
        .collect())
}

/// Parses optional adjacent comment descriptions for `[command.<name>]` tables.
fn parse_command_descriptions(source: &str) -> Option<IndexMap<String, Option<String>>> {
    let document = source.parse::<DocumentMut>().ok()?;
    let commands = document.get("command")?.as_table()?;

    Some(
        commands
            .iter()
            .filter_map(|(name, item)| {
                let table = item.as_table()?;
                Some((
                    String::from(name),
                    table_description(table.decor().prefix()),
                ))
            })
            .collect(),
    )
}

/// Extracts the single physical comment line directly before a table header.
fn table_description(prefix: Option<&RawString>) -> Option<String> {
    let prefix = prefix?.as_str()?;
    let line = prefix.rsplit('\n').nth(1).unwrap_or(prefix);
    let comment = line.trim_end_matches('\r').trim_start().strip_prefix('#')?;
    let description = comment.trim();

    if description.is_empty() {
        return None;
    }

    Some(String::from(description))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use indexmap::IndexMap;
    use toml_edit::RawString;

    use crate::command_info::{
        CommandInfo, command_info, parse_command_descriptions, table_description,
    };
    use crate::tests::flash_command;

    #[test]
    fn table_description_uses_single_adjacent_comment_line() {
        assert_eq!(
            table_description(Some(&RawString::from("# Build documentation.\n"))),
            Some(String::from("Build documentation."))
        );
    }

    #[test]
    fn table_description_normalizes_indented_comment_lines() {
        assert_eq!(
            table_description(Some(&RawString::from("    #   Build documentation.\n"))),
            Some(String::from("Build documentation."))
        );
    }

    #[test]
    fn table_description_uses_only_last_adjacent_comment_line() {
        assert_eq!(
            table_description(Some(&RawString::from(
                "# Long explanation.\n# Short description.\n",
            ))),
            Some(String::from("Short description."))
        );
    }

    #[test]
    fn table_description_ignores_comments_separated_by_blank_lines() {
        assert_eq!(
            table_description(Some(&RawString::from("# Not attached.\n\n"))),
            None
        );
    }

    #[test]
    fn table_description_ignores_empty_comment_lines() {
        assert_eq!(table_description(Some(&RawString::from("#\n"))), None);
    }

    #[test]
    fn table_description_ignores_missing_comments() {
        assert_eq!(table_description(None), None);
        assert_eq!(table_description(Some(&RawString::from("\n"))), None);
    }

    #[test]
    fn parse_command_descriptions_ignores_unrelated_comments() {
        let descriptions = parse_command_descriptions(
            r#"# Project settings.
[project]
default_profile = "sim"

# Project-local `NuttX` workspace.
[workspace]
root = "workspace"

# Build docs.
[command.docs]
command = "doxide"
args = ["build"]

[command.size]
command = "arm-none-eabi-size"
"#,
        )
        .expect("toml should parse");

        assert_eq!(
            descriptions.get("docs"),
            Some(&Some(String::from("Build docs.")))
        );
        assert_eq!(descriptions.get("size"), Some(&None));
    }

    #[test]
    fn command_info_preserves_configured_command_order_and_presence() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        fs::write(
            temp_dir.path().join("nxus.toml"),
            r#"[project]
default_profile = "sim"

# Build documentation.
[command.docs]
command = "doxide"
args = ["build"]

# Check source formatting.
[command.format-check]
command = "clang-format"
args = ["--dry-run"]

[command.size]
command = "arm-none-eabi-size"
args = ["{elf}"]

# Reserved but still listed.
[command.list]
command = "something"
"#,
        )
        .expect("config should be written");

        let mut commands = IndexMap::new();
        commands.insert(String::from("docs"), flash_command("doxide", &["build"]));
        commands.insert(
            String::from("format-check"),
            flash_command("clang-format", &["--dry-run"]),
        );
        commands.insert(
            String::from("size"),
            flash_command("arm-none-eabi-size", &["{elf}"]),
        );
        commands.insert(String::from("list"), flash_command("something", &[]));

        assert_eq!(
            command_info(temp_dir.path(), &commands).expect("command info should load"),
            vec![
                CommandInfo {
                    name: String::from("docs"),
                    description: Some(String::from("Build documentation.")),
                },
                CommandInfo {
                    name: String::from("format-check"),
                    description: Some(String::from("Check source formatting.")),
                },
                CommandInfo {
                    name: String::from("size"),
                    description: None,
                },
                CommandInfo {
                    name: String::from("list"),
                    description: Some(String::from("Reserved but still listed.")),
                },
            ]
        );
    }

    #[test]
    fn command_info_returns_io_error_when_config_is_missing() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");

        assert!(command_info(temp_dir.path(), &IndexMap::new()).is_err());
    }
}
