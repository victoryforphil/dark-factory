use std::path::{Path, PathBuf};

use crate::error::DarkRustError;

const LOCAL_LOCATOR_PREFIX: &str = "@local://";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocatorKind {
    Local,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalLocator {
    pub canonical_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocatorId {
    Local(LocalLocator),
    Unknown(String),
}

impl LocatorId {
    pub fn parse(raw_locator: &str) -> Result<Self, DarkRustError> {
        let trimmed = raw_locator.trim();

        if let Some(local_path) = trimmed.strip_prefix(LOCAL_LOCATOR_PREFIX) {
            let canonical_path = normalize_local_path(local_path)?;
            return Ok(Self::Local(LocalLocator { canonical_path }));
        }

        Ok(Self::Unknown(trimmed.to_string()))
    }

    pub fn from_host_path(path: &Path, kind: LocatorKind) -> Result<Self, DarkRustError> {
        if !path.is_absolute() {
            return Err(DarkRustError::InvalidLocator {
                message: format!(
                    "expected absolute host path for locator conversion (path={})",
                    path.display()
                ),
            });
        }

        let input = path.to_string_lossy();

        match kind {
            LocatorKind::Local => {
                let canonical_path = normalize_local_path(&input)?;
                Ok(Self::Local(LocalLocator { canonical_path }))
            }
        }
    }

    pub fn to_locator_id(&self) -> String {
        match self {
            Self::Local(local) => format!("{LOCAL_LOCATOR_PREFIX}{}", local.canonical_path),
            Self::Unknown(raw) => raw.clone(),
        }
    }

    pub fn to_host_path(&self) -> Result<PathBuf, DarkRustError> {
        match self {
            Self::Local(local) => {
                if cfg!(windows) {
                    Ok(PathBuf::from(local.canonical_path.replace('/', "\\")))
                } else {
                    Ok(PathBuf::from(local.canonical_path.clone()))
                }
            }
            Self::Unknown(raw) => Err(DarkRustError::InvalidLocator {
                message: format!(
                    "unsupported locator format for host path conversion (locator={})",
                    raw
                ),
            }),
        }
    }
}

fn normalize_local_path(path: &str) -> Result<String, DarkRustError> {
    let with_forward_slashes = path.replace('\\', "/");
    let (drive_prefix, remainder) = split_drive_prefix(&with_forward_slashes);

    if !remainder.starts_with('/') {
        return Err(DarkRustError::InvalidLocator {
            message: format!("expected absolute local path (path={path})"),
        });
    }

    let mut segments: Vec<&str> = Vec::new();

    for segment in remainder.split('/') {
        match segment {
            "" | "." => continue,
            ".." => {
                if !segments.is_empty() {
                    segments.pop();
                }
            }
            _ => segments.push(segment),
        }
    }

    if let Some(drive) = drive_prefix {
        if segments.is_empty() {
            return Ok(format!("{drive}/"));
        }

        return Ok(format!("{drive}/{}", segments.join("/")));
    }

    if segments.is_empty() {
        return Ok("/".to_string());
    }

    Ok(format!("/{}", segments.join("/")))
}

fn split_drive_prefix(path: &str) -> (Option<String>, &str) {
    let bytes = path.as_bytes();

    if bytes.len() >= 2 && bytes[1] == b':' && bytes[0].is_ascii_alphabetic() {
        let drive = String::from_utf8_lossy(&bytes[0..1]).to_ascii_lowercase();
        return (Some(format!("{drive}:")), &path[2..]);
    }

    (None, path)
}

#[cfg(test)]
mod tests {
    use super::{LocatorId, LocatorKind};
    use std::path::Path;

    #[test]
    fn parses_local_locator() {
        let parsed =
            LocatorId::parse("@local:///tmp/demo/../project/").expect("parse local locator");

        assert_eq!(parsed.to_locator_id(), "@local:///tmp/project");
        assert_eq!(
            parsed.to_host_path().expect("convert to host path"),
            Path::new("/tmp/project")
        );
    }

    #[test]
    fn preserves_unknown_locator() {
        let parsed =
            LocatorId::parse("repo://dark-factory/product-a").expect("parse unknown locator");

        assert_eq!(parsed.to_locator_id(), "repo://dark-factory/product-a");
        assert!(parsed.to_host_path().is_err());
    }

    #[test]
    fn converts_host_absolute_path_to_local_locator() {
        let parsed = LocatorId::from_host_path(Path::new("/tmp/project"), LocatorKind::Local)
            .expect("build locator from host path");

        assert_eq!(parsed.to_locator_id(), "@local:///tmp/project");
    }
}
