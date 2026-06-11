use bluetape_rs_core::{ValidationError, require_not_blank};
use bluetape_rs_test::TempDir;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScratchConfig {
    pub prefix: String,
    pub file_name: String,
}

#[derive(Debug)]
pub struct ScratchWorkspace {
    temp_dir: TempDir,
    file_path: PathBuf,
    row_count: usize,
}

#[derive(Debug, Error)]
pub enum ScratchError {
    #[error("validation failed: {0}")]
    Validation(#[from] ValidationError),
    #[error("file name must be relative and stay inside the temp workspace: {0}")]
    UnsafeFileName(String),
    #[error("scratch io failed")]
    Io(#[from] io::Error),
}

impl ScratchWorkspace {
    pub fn temp_path(&self) -> &Path {
        self.temp_dir.path()
    }

    pub fn file_path(&self) -> &Path {
        &self.file_path
    }

    pub fn row_count(&self) -> usize {
        self.row_count
    }

    pub fn close(self) -> io::Result<()> {
        self.temp_dir.close()
    }
}

pub fn write_scratch_rows(
    config: ScratchConfig,
    rows: &[&str],
) -> Result<ScratchWorkspace, ScratchError> {
    let prefix = require_not_blank("prefix", &config.prefix)?;
    let file_name = require_not_blank("file_name", &config.file_name)?;
    reject_path_escape(file_name)?;

    let temp_dir = TempDir::new(prefix)?;
    let file_path = temp_dir.path().join(file_name);
    fs::write(&file_path, rows.join("\n"))?;

    Ok(ScratchWorkspace {
        temp_dir,
        file_path,
        row_count: rows.len(),
    })
}

fn reject_path_escape(file_name: &str) -> Result<(), ScratchError> {
    let path = Path::new(file_name);
    if path.is_absolute() || path.components().count() != 1 {
        return Err(ScratchError::UnsafeFileName(file_name.to_owned()));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_rows_into_isolated_temp_workspace() {
        let workspace = write_scratch_rows(
            ScratchConfig {
                prefix: "order-import".to_owned(),
                file_name: "orders.csv".to_owned(),
            },
            &["id,sku", "ord-1,sku-1"],
        )
        .expect("scratch file should be written");

        assert!(workspace.temp_path().exists());
        assert!(workspace.file_path().exists());
        assert_eq!(workspace.row_count(), 2);
        assert_eq!(
            fs::read_to_string(workspace.file_path()).expect("scratch file should be readable"),
            "id,sku\nord-1,sku-1"
        );

        let temp_path = workspace.temp_path().to_path_buf();
        workspace.close().expect("temp workspace should close");
        assert!(!temp_path.exists());
    }

    #[test]
    fn rejects_blank_prefix() {
        let err = write_scratch_rows(
            ScratchConfig {
                prefix: " ".to_owned(),
                file_name: "orders.csv".to_owned(),
            },
            &[],
        )
        .expect_err("blank prefix should fail");

        assert!(matches!(err, ScratchError::Validation(_)));
    }

    #[test]
    fn rejects_path_escape_file_names() {
        let err = write_scratch_rows(
            ScratchConfig {
                prefix: "order-import".to_owned(),
                file_name: "../orders.csv".to_owned(),
            },
            &[],
        )
        .expect_err("path traversal should fail");

        assert!(matches!(err, ScratchError::UnsafeFileName(_)));
    }
}
