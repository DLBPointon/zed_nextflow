// A very basic implementation of zed plugin for Nextflow LSP
//
// Mostly ripped from PureScript to be honest but this is the furthest i've got!
// Issue is that it isn't available on npm registry

use log::warn;
use std::fs;
use zed_extension_api::{self as zed, serde_json, settings::LspSettings, LanguageServerId, Result};

struct NextflowExtension {
    cached_binary_path: Option<String>,
}

#[derive(Clone)]
struct NlsBinary {
    path: String,
    args: Option<Vec<String>>,
    environment: Option<Vec<(String, String)>>,
}

impl NextflowExtension {
    fn language_server_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<NlsBinary> {
        let mut args: Option<Vec<String>> = None;

        let (platform, _arch) = zed::current_platform();

        let environment = match platform {
            zed::Os::Mac | zed::Os::Linux => Some(worktree.shell_env()),
            zed::Os::Windows => None,
        };

        if let Ok(lsp_settings) = LspSettings::for_worktree("nls", worktree) {
            //nls is how it would be found in the settings.json
            // maybe this is wrong though?
            // "languages": {
            //     "nextflow": {
            //         "formatter": "language_server",
            //         "language_servers": ["nls"]
            //     }
            // },
            if let Some(binary) = lsp_settings.binary {
                args = binary.arguments;
                if let Some(path) = binary.path {
                    return Ok(NlsBinary {
                        path: path.clone(),
                        args,
                        environment,
                    });
                }
            }
        }

        if let Some(path) = worktree.which("nls") {
            return Ok(NlsBinary {
                path,
                args,
                environment,
            });
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(NlsBinary {
                    path: path.clone(),
                    args,
                    environment,
                });
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "nextflow-io/language-server", //github org and repo
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        warn!("{:?}", release); // None of these print anyway

        let extension: &str = match platform {
            zed::Os::Mac | zed::Os::Linux => "tar.gz",
            zed::Os::Windows => "zip",
        };

        let download_url = format!(
            // https://github.com/nextflow-io/language-server/archive/refs/tags/v25.04.1.tar.gz
            "https://github.com/nextflow-io/language-server/archive/refs/tags/v{}.{}",
            release.version, extension
        );

        let version_dir = format!("v{}", release.version); // release tag - v25.04.1
        let binary_path = match platform {
            zed::Os::Mac | zed::Os::Linux => format!("{version_dir}/nls"), // save to: v25.04.1/nls
            zed::Os::Windows => format!("{version_dir}/nls.exe"),
        };

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &download_url, // Download link
                &version_dir,  // Save Dir
                match platform {
                    zed::Os::Mac | zed::Os::Linux => zed::DownloadedFileType::GzipTar,
                    zed::Os::Windows => zed::DownloadedFileType::Zip,
                },
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            zed::make_file_executable(&binary_path)?;

            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(NlsBinary {
            path: binary_path,
            args,
            environment,
        })
    }
}

impl zed::Extension for NextflowExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let nls_binary = self.language_server_binary(language_server_id, worktree)?;
        Ok(zed::Command {
            command: nls_binary.path,
            args: nls_binary.args.unwrap_or_default(),
            env: nls_binary.environment.unwrap_or_default(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree("nls", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }
}

zed::register_extension!(NextflowExtension);
