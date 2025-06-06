// A very basic implementation of zed plugin for Nextflow LSP
//
// Mostly ripped from PureScript to be honest but this is the furthest i've got!
// Issue is that it isn't available on npm registry

use std::{env, fs};
use zed_extension_api::{self as zed, serde_json, Result};

const SERVER_PATH: &str = "node_modules/nextflow-language-server/bin/nextflow-language-server"; // Expected path to search and download to
const PACKAGE_NAME: &str = "@nextflow/language-server"; // Used to build a search path on https://registry.npmjs.org/@nextflow/language-server

struct NextflowExtension {
    did_find_server: bool,
}

impl NextflowExtension {
    fn server_exists(&self) -> bool {
        fs::metadata(SERVER_PATH).map_or(false, |stat| stat.is_file())
    }

    fn server_script_path(&mut self, language_server_id: &zed::LanguageServerId) -> Result<String> {
        let server_exists = self.server_exists();
        if self.did_find_server && server_exists {
            return Ok(SERVER_PATH.to_string());
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let version = zed::npm_package_latest_version(PACKAGE_NAME)?;

        if !server_exists
            || zed::npm_package_installed_version(PACKAGE_NAME)?.as_ref() != Some(&version)
        {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            let result = zed::npm_install_package(PACKAGE_NAME, &version);
            match result {
                Ok(()) => {
                    if !self.server_exists() {
                        Err(format!(
                            "installed package '{PACKAGE_NAME}' did not contain expected path '{SERVER_PATH}'",
                        ))?;
                    }
                }
                Err(error) => {
                    if !self.server_exists() {
                        Err(error)?;
                    }
                }
            }
        }

        self.did_find_server = true;
        Ok(SERVER_PATH.to_string())
    }
}

impl zed::Extension for NextflowExtension {
    fn new() -> Self {
        Self {
            did_find_server: false,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let server_path = self.server_script_path(language_server_id)?;
        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![
                env::current_dir()
                    .unwrap()
                    .join(&server_path)
                    .to_string_lossy()
                    .to_string(),
                "--stdio".to_string(),
            ],
            env: Default::default(),
        })
    }

    // fn language_server_initialization_options(
    //     &mut self,
    //     _language_server_id: &zed::LanguageServerId,
    //     _worktree: &zed::Worktree,
    // ) -> Result<Option<serde_json::Value>> {
    //     Ok(Some(serde_json::json!({
    //         "nextflow": {
    //             "addSpagoSources": true
    //         }
    //     })))
    // }
}

zed::register_extension!(NextflowExtension);
