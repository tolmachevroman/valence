//! `settings.toml` in the app's data folder (M4.4). Read once at launch; the
//! file explains itself, so it is written with comments when missing.

use std::path::{Path, PathBuf};

use serde::Deserialize;

pub const FILE: &str = "settings.toml";

/// Written when there is no settings file yet.
pub const TEMPLATE: &str = r#"# Valence settings. Restart Valence after editing this file.

# A GGUF model file to use instead of the default, Qwen3 4B Instruct 2507
# (Q4_K_M). Only the default is tested for accurate cards; with another
# model, cards may choose sentences less well. Cards are kept per model file.
# model = "/Users/you/Models/another-model.gguf"
"#;

#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// A model file to use instead of the default one.
    pub model: Option<PathBuf>,
}

/// The settings in `path`, or the defaults when there is no file yet. A file
/// that doesn't parse is an error, with TOML's message.
pub fn load(path: &Path) -> Result<Settings, String> {
    match std::fs::read_to_string(path) {
        Ok(text) => toml::from_str(&text).map_err(|e| format!("{}: {e}", path.display())),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Settings::default()),
        Err(e) => Err(format!("{}: {e}", path.display())),
    }
}

/// Write the commented template to `path` unless a file is already there.
pub fn create(path: &Path) -> std::io::Result<()> {
    match std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
    {
        Ok(mut file) => std::io::Write::write_all(&mut file, TEMPLATE.as_bytes()),
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => Ok(()),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dir() -> PathBuf {
        let d = std::env::temp_dir().join(format!(
            "valence-settings-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn a_missing_file_is_the_defaults_and_the_template_parses_to_them() {
        let path = dir().join(FILE);
        assert_eq!(load(&path), Ok(Settings::default()));
        create(&path).unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), TEMPLATE);
        assert_eq!(load(&path), Ok(Settings::default()));
    }

    #[test]
    fn reads_the_model_and_keeps_an_existing_file() {
        let path = dir().join(FILE);
        std::fs::write(&path, "model = \"/models/other.gguf\"\n").unwrap();
        create(&path).unwrap();
        assert_eq!(
            load(&path).unwrap().model,
            Some(PathBuf::from("/models/other.gguf"))
        );
    }

    #[test]
    fn a_broken_file_is_an_error() {
        let path = dir().join(FILE);
        std::fs::write(&path, "model = /no/quotes\n").unwrap();
        let e = load(&path).unwrap_err();
        assert!(e.contains(FILE), "{e}");
    }
}
