//! `CoreML` model builder for convenient model loading

use crate::config::Config;
use crate::CoreMLModel;
use candle_core::Error as CandleError;
use std::path::{Path, PathBuf};

/// Builder for `CoreML` models
///
/// This provides an interface for loading `CoreML` models with configuration
/// management and device selection.
pub struct CoreMLModelBuilder {
    config: Config,
    model_filename: PathBuf,
}

impl CoreMLModelBuilder {
    /// Create a new builder with the specified model path and config
    pub fn new<P: AsRef<Path>>(model_path: P, config: Config) -> Self {
        Self {
            config,
            model_filename: model_path.as_ref().to_path_buf(),
        }
    }

    /// Load a `CoreML` model from `HuggingFace` or local files
    pub fn load_from_hub(
        model_id: &str,
        model_filename: Option<&str>,
        config_filename: Option<&str>,
    ) -> Result<Self, CandleError> {
        use crate::get_local_or_remote_file;
        use hf_hub::{api::sync::Api, Repo, RepoType};

        let api =
            Api::new().map_err(|e| CandleError::Msg(format!("Failed to create HF API: {e}")))?;
        let repo = api.repo(Repo::with_revision(
            model_id.to_string(),
            RepoType::Model,
            "main".to_string(),
        ));

        // Load config
        let config_path = match config_filename {
            Some(filename) => get_local_or_remote_file(filename, &repo)
                .map_err(|e| CandleError::Msg(format!("Failed to get config file: {e}")))?,
            None => get_local_or_remote_file("config.json", &repo)
                .map_err(|e| CandleError::Msg(format!("Failed to get config.json: {e}")))?,
        };

        let config_str = std::fs::read_to_string(config_path)
            .map_err(|e| CandleError::Msg(format!("Failed to read config file: {e}")))?;
        let config: Config = serde_json::from_str(&config_str)
            .map_err(|e| CandleError::Msg(format!("Failed to parse config: {e}")))?;

        // Get model file
        let model_path = match model_filename {
            Some(filename) => get_local_or_remote_file(filename, &repo)
                .map_err(|e| CandleError::Msg(format!("Failed to get model file: {e}")))?,
            None => {
                // Try common CoreML model filenames
                for filename in &["model.mlmodelc", "model.mlpackage"] {
                    if let Ok(path) = get_local_or_remote_file(filename, &repo) {
                        return Ok(Self::new(path, config));
                    }
                }
                return Err(CandleError::Msg("No CoreML model file found".to_string()));
            }
        };

        Ok(Self::new(model_path, config))
    }

    /// Build the CoreML model
    pub fn build_model(&self) -> Result<CoreMLModel, CandleError> {
        CoreMLModel::load_from_file(&self.model_filename, &self.config)
    }

    /// Get the config
    pub fn config(&self) -> &Config {
        &self.config
    }
}
