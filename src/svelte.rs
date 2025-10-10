use std::{collections::HashSet, env};
use zed_extension_api::{self as zed, serde_json, Result};

struct SvelteExtension {
    installed: HashSet<String>,
}

const PACKAGE_NAME: &str = "svelte-language-server";
const TS_PLUGIN_PACKAGE_NAME: &str = "typescript-svelte-plugin";
const MCP_SERVER_PACKAGE_NAME: &str = "@sveltejs/mcp";

impl SvelteExtension {
    fn install_package_if_needed(
        &mut self,
        id: Option<&zed::LanguageServerId>,
        package_name: &str,
    ) -> Result<()> {
        let installed_version = zed::npm_package_installed_version(package_name)?;

        // If package is already installed in this session, then we won't reinstall it
        if installed_version.is_some() && self.installed.contains(package_name) {
            return Ok(());
        }

        if let Some(id) = id {
            zed::set_language_server_installation_status(
                id,
                &zed::LanguageServerInstallationStatus::CheckingForUpdate,
            );
        }

        let latest_version = zed::npm_package_latest_version(package_name)?;

        if installed_version.as_ref() != Some(&latest_version) {
            println!("Installing {package_name}@{latest_version}...");

            if let Some(id) = id {
                zed::set_language_server_installation_status(
                    id,
                    &zed::LanguageServerInstallationStatus::Downloading,
                );
            }

            if let Err(error) = zed::npm_install_package(package_name, &latest_version) {
                // If installation failed, but we don't want to error but rather reuse existing version
                if installed_version.is_none() {
                    Err(error)?;
                }
            }
        } else {
            println!("Found {package_name}@{latest_version} installed");
        }

        self.installed.insert(package_name.into());
        Ok(())
    }
}

impl zed::Extension for SvelteExtension {
    fn new() -> Self {
        Self {
            installed: HashSet::new(),
        }
    }

    fn language_server_command(
        &mut self,
        id: &zed::LanguageServerId,
        _: &zed::Worktree,
    ) -> Result<zed::Command> {
        self.install_package_if_needed(Some(id), PACKAGE_NAME)?;
        self.install_package_if_needed(Some(id), TS_PLUGIN_PACKAGE_NAME)?;

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![
                env::current_dir()
                    .unwrap()
                    .join("node_modules")
                    .join(PACKAGE_NAME)
                    .join("bin/server.js")
                    .to_string_lossy()
                    .to_string(),
                "--stdio".to_string(),
            ],
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _: &zed::LanguageServerId,
        _: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let config = serde_json::json!({
          "inlayHints": {
            "parameterNames": {
              "enabled": "all",
              "suppressWhenArgumentMatchesName": false
            },
            "parameterTypes": {
              "enabled": true
            },
            "variableTypes": {
              "enabled": true,
              "suppressWhenTypeMatchesName": false
            },
            "propertyDeclarationTypes": {
              "enabled": true
            },
            "functionLikeReturnTypes": {
              "enabled": true
            },
            "enumMemberValues": {
              "enabled": true
            }
          }
        });

        Ok(Some(serde_json::json!({
            "provideFormatter": true,
            "dontFilterIncompleteCompletions": true,
            "configuration": {
                "typescript": config,
                "javascript": config
            }
        })))
    }

    fn language_server_additional_workspace_configuration(
        &mut self,
        _id: &zed::LanguageServerId,
        target_id: &zed::LanguageServerId,
        _: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        match target_id.as_ref() {
            "vtsls" => Ok(Some(serde_json::json!({
                "vtsls": {
                    "tsserver": {
                        "globalPlugins": [{
                            "name": TS_PLUGIN_PACKAGE_NAME,
                            "location": env::current_dir().unwrap()
                                .join("node_modules")
                                .join(&TS_PLUGIN_PACKAGE_NAME)
                                .to_string_lossy()
                                .to_string(),
                            "enableForWorkspaceTypeScriptVersions": true
                        }]
                    }
                },
            }))),
            _ => Ok(None),
        }
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &zed_extension_api::ContextServerId,
        _project: &zed_extension_api::Project,
    ) -> Result<zed::Command> {
        self.install_package_if_needed(None, MCP_SERVER_PACKAGE_NAME)?;

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![env::current_dir()
                .unwrap()
                .join("node_modules")
                .join(MCP_SERVER_PACKAGE_NAME)
                .join("dist/index.js")
                .to_string_lossy()
                .to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(SvelteExtension);
