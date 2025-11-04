mod runtime;

use std::{collections::HashSet, env};
use zed_extension_api::{self as zed, Result, serde_json};
use runtime::Runtime;

struct SvelteExtension {
    installed: HashSet<String>,
    runtime: Runtime
}

const PACKAGE_NAME: &str = "svelte-language-server";
const TS_PLUGIN_PACKAGE_NAME: &str = "typescript-svelte-plugin";

impl SvelteExtension {
    fn install_package_if_needed(
        &mut self,
        id: &zed::LanguageServerId,
        package_name: &str,
    ) -> Result<()> {
        let installed_version = self.runtime.installed_package_version(package_name)?;

        // If package is already installed in this session, then we won't reinstall it
        if installed_version.is_some() && self.installed.contains(package_name) {
            return Ok(());
        }

        zed::set_language_server_installation_status(
            id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let latest_version = self.runtime.latest_package_version(package_name)?;

        if installed_version.as_ref() != Some(&latest_version) {
            println!("Installing {package_name}@{latest_version}...");

            zed::set_language_server_installation_status(
                id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            if let Err(error) = self.runtime.install_package(package_name, &latest_version){
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
            runtime: Runtime::new()
        }
    }

    fn language_server_command(
        &mut self,
        id: &zed::LanguageServerId,
        _: &zed::Worktree,
    ) -> Result<zed::Command> {
        self.install_package_if_needed(id, PACKAGE_NAME)?;
        self.install_package_if_needed(id, TS_PLUGIN_PACKAGE_NAME)?;

        let server_path = env::current_dir()
            .unwrap()
            .join("node_modules")
            .join(PACKAGE_NAME)
            .join("bin/server.js")
            .to_string_lossy()
            .to_string();

        self.runtime.server_command(&server_path)

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
}

zed::register_extension!(SvelteExtension);
