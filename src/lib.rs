use schemars::JsonSchema;
use serde::Deserialize;
use zed_extension_api::{
    self as zed, settings::ContextServerSettings, Command, ContextServerConfiguration,
    ContextServerId, Project, Result,
};

const PACKAGE_NAME: &str = "mcp-remote";
const PACKAGE_VERSION: &str = "latest";
const BIN_PATH: &str = "node_modules/.bin/mcp-remote";

#[derive(Debug, Deserialize, JsonSchema)]
struct YaakMcpSettings {
    /// URL of the Yaak MCP server (default: http://127.0.0.1:64343/mcp)
    server_url: Option<String>,
}

struct YaakMcpExtension;

impl zed::Extension for YaakMcpExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let version = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if version.is_none() {
            zed::npm_install_package(PACKAGE_NAME, PACKAGE_VERSION)?;
        }

        let settings = ContextServerSettings::for_project("mcp-yaak", project)?;
        let server_url = settings
            .settings
            .and_then(|s| serde_json::from_value::<YaakMcpSettings>(s).ok())
            .and_then(|s| s.server_url)
            .unwrap_or_else(|| "http://127.0.0.1:64343/mcp".to_string());

        let is_http = server_url.starts_with("http://");
        let mut args = vec![server_url];

        if is_http {
            args.push("--allow-http".to_string());
        }

        zed::make_file_executable(BIN_PATH)?;

        Ok(Command {
            command: BIN_PATH.to_string(),
            args,
            env: vec![],
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        Ok(Some(ContextServerConfiguration {
            installation_instructions: include_str!(
                "../configuration/installation_instructions.md"
            )
            .to_string(),
            default_settings: include_str!("../configuration/default_settings.jsonc").to_string(),
            settings_schema: serde_json::to_string(&schemars::schema_for!(YaakMcpSettings))
                .map_err(|e| e.to_string())?,
        }))
    }
}

zed::register_extension!(YaakMcpExtension);
