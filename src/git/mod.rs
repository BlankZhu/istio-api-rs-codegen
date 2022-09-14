use std::path::PathBuf;
use std::process::Command;

use thiserror::Error;

use crate::constant;

pub struct Gitter {
    repository: PathBuf,
}

impl Gitter {
    pub fn new(repository: PathBuf) -> Self {
        return Gitter { repository };
    }

    pub fn checkout_tag(&self, tag: &str) -> anyhow::Result<()> {
        let path = self.repository.display().to_string();
        let checkout_tag = "tags/".to_string() + tag;

        let output = match Command::new(constant::GIT_COMMAND)
            .args(["-C", path.as_str(), "checkout", checkout_tag.as_str()])
            .output()
        {
            Ok(o) => o,
            Err(e) => {
                let err = GitterError::CommandError {
                    subcommand: "checkout".to_string(),
                    detail: format!("{}", e),
                };
                anyhow::bail!("{}", err);
            }
        };

        if !output.status.success() {
            let detail = String::from_utf8_lossy(&output.stderr);
            let err = GitterError::CheckoutError {
                detail: detail.into(),
            };
            anyhow::bail!("{}", err);
        }

        Ok(())
    }

    pub fn restore(&self) -> anyhow::Result<()> {
        let path = self.repository.display().to_string();

        let output = match Command::new(constant::GIT_COMMAND)
            .args(["-C", path.as_str(), "restore", "."])
            .output()
        {
            Ok(o) => o,
            Err(e) => {
                let err = GitterError::CommandError {
                    subcommand: "restore".to_string(),
                    detail: format!("{}", e),
                };
                anyhow::bail!("{}", err);
            }
        };

        if !output.status.success() {
            let detail = String::from_utf8_lossy(&output.stderr);
            let err = GitterError::RestoreError {
                detail: detail.into(),
            };
            anyhow::bail!("{}", err);
        }

        Ok(())
    }

    pub fn force_clean(&self) -> anyhow::Result<()> {
        let path = self.repository.display().to_string();

        let output = match Command::new(constant::GIT_COMMAND)
            .args(["-C", path.as_str(), "clean", "-f"])
            .output()
        {
            Ok(o) => o,
            Err(e) => {
                let err = GitterError::CommandError {
                    subcommand: "restore".to_string(),
                    detail: format!("{}", e),
                };
                anyhow::bail!("{}", err);
            }
        };

        if !output.status.success() {
            let detail = String::from_utf8_lossy(&output.stderr);
            let err = GitterError::ForceCleanError {
                detail: detail.into(),
            };
            anyhow::bail!("{}", err);
        }

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum GitterError {
    #[error("git command `{subcommand:?}` execution failed: {detail:?}")]
    CommandError { subcommand: String, detail: String },
    #[error("git checkout failed: {detail:?}")]
    CheckoutError { detail: String },
    #[error("git restore failed: {detail:?}")]
    RestoreError { detail: String },
    #[error("git clean (force) failed: {detail:?}")]
    ForceCleanError { detail: String },
}
