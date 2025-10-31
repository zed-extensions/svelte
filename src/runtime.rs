use std::path::PathBuf;
use std::process::Command as SystemCommand;
use zed_extension_api::{self as zed, Result};
use which::which;

pub enum Runtime {
    Bun(PathBuf),
    Node
}

impl Runtime {
    pub fn new() -> Self {
        match which("bun") {
            Ok(path) => {
                println!("Bun detected at {:?}, using it as the runtime.", path);
                Self::Bun(path)
            }
            Err(_) => {
                println!("Bun not found. Falling back to Zed's built-in Node.js runtime.");
                Self::Node
            }
        }
    }

    pub fn server_command(&self, server_path: &str) -> Result<zed::Command> {
        let command = match self {
            Runtime::Bun(path) => path.to_string_lossy().to_string(),
            Runtime::Node => zed::node_binary_path()?
        };

        Ok(zed::Command {
            command,
            args: vec![server_path.to_string(), "--stdio".to_string()],
            env: Default::default()
        })
    }

    pub fn install_package(&self, package_name: &str, version: &str) -> Result<()> {
        println!("Installing {}@{} using {:?}...", package_name, version, self);
        match self {
            Runtime::Bun(path) => {
                let exit_status = SystemCommand::new(path)
                    .arg("add")
                    .arg(format!("{}@{}", package_name, version))
                    .status()
                    .map_err(|e| format!("Failed to execute bun: {}", e))?;

                if !exit_status.success() {
                    return Err(format!("'bun add' failed with status: {}", exit_status).into())
                }
            }
            Runtime::Node => {
                zed::npm_install_package(package_name, version)?;
            }
        };
        Ok(())
    }

    pub fn latest_package_version(&self, package_name: &str) -> Result<String>{
        match self {
            Runtime::Bun(_) | Runtime::Node => zed::npm_package_latest_version(package_name)
        }
    }

    pub fn installed_package_version(&self, package_name: &str) -> Result<Option<String>> {
        zed::npm_package_installed_version(package_name)
    }
}

impl std::fmt::Debug for Runtime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bun(path) => write!(f, "Bun({:?})", path),
            Self::Node => write!(f, "Node"),
        }
    }
}
