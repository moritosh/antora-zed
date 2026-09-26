use std::fs;

use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result};

const SERVER_NAME: &str = "antora-ls";
/// GitHub repo that publishes prebuilt `antora-ls` binaries (see README).
const GITHUB_REPO: &str = "moritosh/antora-ls";

struct AntoraExtension {
    cached_binary_path: Option<String>,
}

impl AntoraExtension {
    fn binary_path(&mut self, id: &LanguageServerId, worktree: &zed::Worktree) -> Result<String> {
        // 1. Explicit path in Zed settings: lsp.antora-ls.binary.path
        if let Some(path) = LspSettings::for_worktree(SERVER_NAME, worktree)
            .ok()
            .and_then(|s| s.binary)
            .and_then(|b| b.path)
        {
            return Ok(path);
        }

        // 2. Anything on PATH (e.g. `cargo install --path antora-ls`).
        if let Some(path) = worktree.which(SERVER_NAME) {
            return Ok(path);
        }

        // 3. Previously downloaded copy.
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).is_ok_and(|m| m.is_file()) {
                return Ok(path.clone());
            }
        }

        // 4. Download the latest release for this platform.
        zed::set_language_server_installation_status(
            id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let release = zed::latest_github_release(
            GITHUB_REPO,
            zed::GithubReleaseOptions { require_assets: true, pre_release: false },
        )?;

        let (os, arch) = zed::current_platform();
        let arch = match arch {
            zed::Architecture::Aarch64 => "aarch64",
            zed::Architecture::X8664 => "x86_64",
            zed::Architecture::X86 => return Err("32-bit x86 is not supported".into()),
        };
        let (target, archive, file_type, exe) = match os {
            zed::Os::Mac => ("apple-darwin", "tar.gz", zed::DownloadedFileType::GzipTar, ""),
            zed::Os::Linux => ("unknown-linux-gnu", "tar.gz", zed::DownloadedFileType::GzipTar, ""),
            zed::Os::Windows => ("pc-windows-msvc", "zip", zed::DownloadedFileType::Zip, ".exe"),
        };
        let asset_name = format!("{SERVER_NAME}-{arch}-{target}.{archive}");
        let asset = release
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .ok_or_else(|| format!("no release asset named {asset_name}"))?;

        let version_dir = format!("{SERVER_NAME}-{}", release.version);
        let binary_path = format!("{version_dir}/{SERVER_NAME}{exe}");

        if !fs::metadata(&binary_path).is_ok_and(|m| m.is_file()) {
            zed::set_language_server_installation_status(
                id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            zed::download_file(&asset.download_url, &version_dir, file_type)
                .map_err(|e| format!("failed to download {asset_name}: {e}"))?;
            zed::make_file_executable(&binary_path)?;

            // Remove older versions.
            if let Ok(entries) = fs::read_dir(".") {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().into_owned();
                    if name.starts_with(SERVER_NAME) && name != version_dir {
                        fs::remove_dir_all(entry.path()).ok();
                    }
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for AntoraExtension {
    fn new() -> Self {
        Self { cached_binary_path: None }
    }

    fn language_server_command(
        &mut self,
        id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary = LspSettings::for_worktree(SERVER_NAME, worktree).ok().and_then(|s| s.binary);
        let args = binary.as_ref().and_then(|b| b.arguments.clone()).unwrap_or_default();
        let mut env = worktree.shell_env();
        if let Some(extra) = binary.and_then(|b| b.env) {
            env.extend(extra);
        }
        Ok(zed::Command { command: self.binary_path(id, worktree)?, args, env })
    }
}

zed::register_extension!(AntoraExtension);
