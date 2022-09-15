use std::fs;
use std::path::PathBuf;

use serde::Deserialize;
use serde_yaml::{Deserializer, Mapping, Sequence, Value};
use thiserror::Error;

pub struct Cutter {
    cue_yaml_path: PathBuf,
}

impl Cutter {
    pub fn new(cue_yaml_path: PathBuf) -> Self {
        Cutter { cue_yaml_path }
    }

    pub fn modify_cue_file(&self) -> anyhow::Result<()> {
        let content = self.read_cue_yaml_file()?;
        let docs = Deserializer::from_str(content.as_str());
        let mut yamls = Vec::new();
        for doc in docs {
            let rv = Value::deserialize(doc).map_err(|e| CutterError::DeserializeCueError {
                detail: format!("{}", e),
            })?;
            yamls.push(rv);
        }
        if yamls.len() != 1 {
            let err = CutterError::DeserializeCueError {
                detail: format!(
                    "cue yaml file at `{}` doesn't contains a single yaml doc",
                    self.cue_yaml_path.display()
                ),
            };
            anyhow::bail!("{}", err);
        }

        let doc = yamls.get(0).unwrap();
        let new_doc = self.add_directories(doc)?;
        let new_content =
            serde_yaml::to_string(&new_doc).map_err(|e| CutterError::SerializeCueError {
                detail: format!("{}", e),
            })?;

        self.save_cue_yaml_file(new_content)?;
        Ok(())
    }

    fn read_cue_yaml_file(&self) -> Result<String, CutterError> {
        fs::read_to_string(self.cue_yaml_path.as_path()).map_err(|e| CutterError::ReadCueError {
            path: self.cue_yaml_path.display().to_string(),
            detail: format!("{}", e),
        })
    }

    fn save_cue_yaml_file(&self, content: String) -> Result<(), CutterError> {
        fs::write(self.cue_yaml_path.as_path(), content).map_err(|e| CutterError::WriteCueError {
            path: self.cue_yaml_path.display().to_string(),
            detail: format!("{}", e),
        })
    }

    fn add_directories(&self, cue_doc: &Value) -> Result<Value, CutterError> {
        let mut doc = cue_doc.clone();

        let directories = match doc.get_mut("directories") {
            Some(dirs) => dirs,
            None => {
                let err = CutterError::FieldMissingError {
                    field: "directories".to_string(),
                };
                return Err(err);
            }
        };
        let directories = match directories.as_mapping_mut() {
            Some(dirs) => dirs,
            None => {
                let err = CutterError::FieldTypeError {
                    field: "directories".to_string(),
                    expect: "mapping".to_string(),
                };
                return Err(err);
            }
        };
        self.add_telemetry_section(directories);
        self.add_operator_section(directories);

        Ok(doc)
    }

    fn add_telemetry_section(&self, directories: &mut Mapping) {
        let dir_key = Value::String("telemetry/v1alpha1".to_string());
        let mode_key = Value::String("mode".to_string());
        let mode_value = Value::String("perFile".to_string());
        let mut mode_obj = Mapping::new();
        mode_obj.insert(mode_key, mode_value);
        let mut modes = Sequence::new();
        modes.push(mode_obj.into());

        directories.insert(dir_key, modes.into());
    }

    fn add_operator_section(&self, directories: &mut Mapping) {
        let dir_key = Value::String("operator/v1alpha1".to_string());
        let mode_key = Value::String("mode".to_string());
        let mode_value = Value::String("perFile".to_string());
        let mut mode_obj = Mapping::new();
        mode_obj.insert(mode_key, mode_value);
        let mut modes = Sequence::new();
        modes.push(mode_obj.into());

        directories.insert(dir_key, modes.into());
    }
}

#[derive(Error, Debug)]
pub enum CutterError {
    #[error("read cue file `{path:?}` failed: {detail:?}")]
    ReadCueError { path: String, detail: String },

    #[error("write cue file `{path:?}` failed: {detail:?}")]
    WriteCueError { path: String, detail: String },

    #[error("deserialize cue yaml content failed: {detail:?}")]
    DeserializeCueError { detail: String },

    #[error("serialize cue yaml content failed: {detail:?}")]
    SerializeCueError { detail: String },

    #[error("field `{field:?}` missing")]
    FieldMissingError { field: String },

    #[error("field `{field:?}` type not compatible, expect: {expect:?}")]
    FieldTypeError { field: String, expect: String },
}
