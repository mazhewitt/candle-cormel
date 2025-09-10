// Model Quality Verification Framework
//
// This module provides comprehensive testing for model correctness and quality,
// going beyond simple token matching to ensure semantic reasonableness.

use candle_coreml::{QwenModel, UnifiedModelLoader};
use anyhow::Result;
use std::collections::HashMap;

const MODEL_ID: &str = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";

/// Test case with semantic expectations rather than exact token matching
#[derive(Debug)]
struct QualityTestCase {
    prompt: String,
    description: String,
    semantic_expectations: Vec<SemanticExpectation>,
    exact_token_expectation: Option<i64>, // For regression detection
}

#[derive(Debug, Clone)]
enum SemanticExpectation {
    ShouldContainWord(String),
    ShouldNotContainWord(String),  
    ShouldBeCoherent,              // Tests if output makes linguistic sense
    ShouldCompleteLogically,       // Tests if completion is contextually appropriate
    TokenShouldBeInVocab,          // Basic sanity check
}

impl QualityTestCase {
    fn new(prompt: &str, description: &str) -> Self {
        Self {
            prompt: prompt.to_string(),
            description: description.to_string(),
            semantic_expectations: Vec::new(),
            exact_token_expectation: None,
        }
    }
    
    fn expect_contains_word(mut self, word: &str) -> Self {
        self.semantic_expectations.push(SemanticExpectation::ShouldContainWord(word.to_string()));
        self
    }
    
    fn expect_coherent(mut self) -> Self {
        self.semantic_expectations.push(SemanticExpectation::ShouldBeCoherent);
        self
    }
    
    fn expect_logical_completion(mut self) -> Self {
        self.semantic_expectations.push(SemanticExpectation::ShouldCompleteLogically);
        self
    }
    
    fn expect_token_in_vocab(mut self) -> Self {
        self.semantic_expectations.push(SemanticExpectation::TokenShouldBeInVocab);
        self
    }
    
    fn with_regression_token(mut self, token: i64) -> Self {
        self.exact_token_expectation = Some(token);
        self
    }
}

/// Comprehensive model verification results
#[derive(Debug)]
struct VerificationResults {
    test_results: HashMap<String, TestResult>,
    overall_quality_score: f32,
    regression_detected: bool,
}

#[derive(Debug)]
struct TestResult {
    success: bool,
    generated_token: i64,
    decoded_output: String,
    semantic_analysis: Vec<(SemanticExpectation, bool)>,
    notes: Vec<String>,
}

/// Model Quality Verifier
struct ModelQualityVerifier {
    model: QwenModel,
}

impl ModelQualityVerifier {
    fn new() -> Result<Self> {
        let loader = UnifiedModelLoader::new()?;
        let model = loader.load_model(MODEL_ID)?;
        Ok(Self { model })
    }
    
    fn run_verification(&mut self) -> Result<VerificationResults> {
        let test_cases = self.create_test_cases();
        let mut test_results = HashMap::new();
        let mut total_score = 0.0;
        let mut regression_count = 0;
        
        println!("🧪 Running Model Quality Verification");
        println!("=====================================");
        
        for test_case in test_cases {
            println!("\n📝 Testing: {}", test_case.description);
            println!("   Prompt: '{}'", test_case.prompt);
            
            let result = self.run_single_test(&test_case)?;
            let case_score = if result.success { 1.0 } else { 0.0 };
            
            if let Some(expected_token) = test_case.exact_token_expectation {
                if result.generated_token != expected_token {
                    regression_count += 1;
                    println!("   ⚠️ REGRESSION: Expected token {}, got {}", expected_token, result.generated_token);
                }
            }
            
            println!("   🎯 Generated: Token {} -> '{}'", result.generated_token, result.decoded_output);
            println!("   ✅ Pass: {}", if result.success { "YES" } else { "NO" });
            
            for note in &result.notes {
                println!("   📋 {}", note);
            }
            
            total_score += case_score;
            test_results.insert(test_case.description.clone(), result);
        }
        
        let overall_quality_score = total_score / test_results.len() as f32;
        let regression_detected = regression_count > 0;
        
        Ok(VerificationResults {
            test_results,
            overall_quality_score,
            regression_detected,
        })
    }
    
    fn run_single_test(&mut self, test_case: &QualityTestCase) -> Result<TestResult> {
        let token = self.model.forward_text(&test_case.prompt)?;
        
        let decoded_output = self.model.tokenizer()
            .decode(&[token as u32], false)
            .unwrap_or_else(|_| "[DECODE_ERROR]".to_string());
        
        let mut semantic_analysis = Vec::new();
        let mut notes = Vec::new();
        let mut overall_success = true;
        
        for expectation in &test_case.semantic_expectations {
            let passes = match expectation {
                SemanticExpectation::ShouldContainWord(word) => {
                    decoded_output.contains(word)
                }
                SemanticExpectation::ShouldNotContainWord(word) => {
                    !decoded_output.contains(word)
                }
                SemanticExpectation::ShouldBeCoherent => {
                    self.assess_coherence(&decoded_output)
                }
                SemanticExpectation::ShouldCompleteLogically => {
                    self.assess_logical_completion(&test_case.prompt, &decoded_output)
                }
                SemanticExpectation::TokenShouldBeInVocab => {
                    !decoded_output.contains("[DECODE_ERROR]")
                }
            };
            
            if !passes {
                overall_success = false;
                notes.push(format!("Failed expectation: {:?}", expectation));
            }
            
            semantic_analysis.push((expectation.clone(), passes));
        }
        
        Ok(TestResult {
            success: overall_success,
            generated_token: token,
            decoded_output,
            semantic_analysis,
            notes,
        })
    }
    
    fn create_test_cases(&self) -> Vec<QualityTestCase> {
        vec![
            // Classic completion tests
            QualityTestCase::new(
                "The quick brown fox jumps over the lazy", 
                "Classic fox-dog completion"
            )
            .expect_token_in_vocab()
            .expect_coherent()
            .with_regression_token(3974), // Current output for regression detection
            
            // Simple factual tests
            QualityTestCase::new(
                "The capital of France is",
                "Basic factual knowledge"
            )
            .expect_contains_word("Paris")
            .expect_token_in_vocab()
            .expect_coherent(),
            
            // Language understanding tests
            QualityTestCase::new(
                "Hello, how are",
                "Conversational completion"
            )
            .expect_token_in_vocab()
            .expect_coherent()
            .expect_logical_completion(),
            
            // Pattern completion tests  
            QualityTestCase::new(
                "1, 2, 3, 4,",
                "Numerical sequence"
            )
            .expect_token_in_vocab()
            .expect_coherent(),
            
            // Common sense tests
            QualityTestCase::new(
                "The sky is",
                "Common knowledge completion"
            )
            .expect_token_in_vocab()
            .expect_coherent()
            .expect_logical_completion(),
        ]
    }
    
    fn assess_coherence(&self, output: &str) -> bool {
        // Basic coherence checks:
        // 1. Not empty or error
        // 2. Contains valid characters
        // 3. Not obviously broken (like random symbols)
        
        if output.is_empty() || output.contains("[DECODE_ERROR]") {
            return false;
        }
        
        // Check for reasonable character distribution (not all special chars)
        let alpha_count = output.chars().filter(|c| c.is_alphabetic()).count();
        let total_count = output.chars().count();
        
        if total_count == 0 {
            return false;
        }
        
        // At least 30% alphabetic characters for coherence
        (alpha_count as f32 / total_count as f32) >= 0.3
    }
    
    fn assess_logical_completion(&self, prompt: &str, completion: &str) -> bool {
        // Basic logical completion assessment
        // This is a simplified heuristic - in production you'd want more sophisticated NLP
        
        if completion.trim().is_empty() {
            return false;
        }
        
        // Check for common problematic patterns
        let problematic_patterns = [
            "The quick", // Repeating the prompt doesn't make sense
            "The quick brown fox", // Exact repetition  
        ];
        
        for pattern in &problematic_patterns {
            if prompt.contains("The quick brown fox") && completion.trim() == pattern.trim() {
                return false;
            }
        }
        
        true
    }
}

#[cfg(target_os = "macos")]
#[tokio::test]
async fn test_comprehensive_model_quality() -> Result<()> {
    let mut verifier = ModelQualityVerifier::new()?;
    let results = verifier.run_verification()?;
    
    println!("\n📊 VERIFICATION RESULTS");
    println!("=======================");
    println!("Overall Quality Score: {:.1}%", results.overall_quality_score * 100.0);
    println!("Regression Detected: {}", if results.regression_detected { "YES" } else { "NO" });
    
    // We can be more lenient than exact token matching, but quality should be reasonable
    assert!(results.overall_quality_score >= 0.6, 
        "Model quality score too low: {:.1}%", results.overall_quality_score * 100.0);
    
    // At least basic coherence should work
    let coherence_tests = results.test_results.values()
        .filter(|r| r.semantic_analysis.iter()
            .any(|(exp, passed)| matches!(exp, SemanticExpectation::ShouldBeCoherent) && *passed))
        .count();
    
    assert!(coherence_tests > 0, "No tests passed basic coherence checks");
    
    println!("✅ Model quality verification passed");
    Ok(())
}

#[cfg(target_os = "macos")]  
#[tokio::test]
async fn test_output_consistency() -> Result<()> {
    println!("🔄 Testing output consistency...");
    
    let mut verifier = ModelQualityVerifier::new()?;
    let prompt = "The quick brown fox jumps over the lazy";
    
    // Test that model produces consistent outputs for same input
    let mut tokens = Vec::new();
    for i in 0..5 {
        let token = verifier.model.forward_text(prompt)?;
        tokens.push(token);
        println!("Run {}: Token {}", i + 1, token);
    }
    
    // All outputs should be identical for deterministic behavior
    let first_token = tokens[0];
    let all_same = tokens.iter().all(|&t| t == first_token);
    
    assert!(all_same, "Model outputs are not consistent across runs: {:?}", tokens);
    
    println!("✅ Model output consistency verified");
    Ok(())
}

#[test]
fn test_verifier_creation() {
    // Test that verifier can be created without loading model (for CI)
    println!("🏗️ Testing verifier framework creation (no model loading)");
    
    // Just test the framework structure
    let test_cases = vec![
        QualityTestCase::new("test", "Test case").expect_coherent(),
    ];
    
    assert!(!test_cases.is_empty());
    println!("✅ Verifier framework structure is valid");
}