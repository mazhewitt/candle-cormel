//! Smoke tests for CoreML metadata extraction on a real .mlpackage in the repo
//!
//! These are ignored by default because they rely on local Python/coremltools
//! and macOS CoreML availability. Run with: cargo test -- --ignored

#[cfg(target_os = "macos")]
mod coreml_metadata_extraction_tests {
    use candle_coreml::config_generator::CoreMLMetadataExtractor;
    use std::path::PathBuf;

    /// Ensure we can extract some non-empty input/output tensors from the sample Mistral package
    #[test]
    #[ignore]
    fn extract_signatures_from_mistral_package() {
        // Path to the sample package included in the repo
        let package_dir = PathBuf::from("mistral-model/StatefulMistral7BInstructInt4.mlpackage");
        let model_path = package_dir.join("Data/com.apple.CoreML/model.mlmodel");

        assert!(model_path.exists(), "Expected model file at {}", model_path.display());

        let extractor = CoreMLMetadataExtractor::new();
        if !extractor.is_coremltools_available() {
            eprintln!("coremltools not available; skipping smoke extraction test");
            return; // don't fail in environments without coremltools
        }

        let (inputs, outputs) = extractor
            .extract_tensor_signatures(&model_path)
            .expect("failed to extract tensor signatures via coremltools/native");

        // We don't assert exact names, just that there's something meaningful
        assert!(
            !inputs.is_empty() || !outputs.is_empty(),
            "Expected some inputs or outputs, got none"
        );

        eprintln!(
            "Extracted {} inputs and {} outputs: inputs={:?} outputs={:?}",
            inputs.len(),
            outputs.len(),
            inputs.keys().collect::<Vec<_>>(),
            outputs.keys().collect::<Vec<_>>()
        );
    }
}
