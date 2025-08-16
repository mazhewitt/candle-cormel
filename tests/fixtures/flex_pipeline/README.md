# Test Generation Suite

This directory contains a complete test data generation suite for the **Qwen ANE Typo Fixer** pipeline, producing test data that exactly matches the behavior of `typo_fixer_complete.py`.

## 🎯 Purpose

Generate structured test data for each stage of the typo correction pipeline to enable:
- **Independent testing** of pipeline components
- **Validation** of new model implementations 
- **Debugging** and performance analysis
- **Integration testing** across different frameworks
- **Regression testing** when modifying the pipeline

## 🏗️ Model Architecture

### **Base Model**
- **Model**: `Qwen/Qwen3-0.6B` (596M parameters)
- **Fine-tuned**: `mazhewitt/qwen-typo-fixer` 
- **Performance**: 88.5% sentence accuracy on typo correction
- **Deployment**: CoreML models optimized for Apple Neural Engine

### **Multi-Component Pipeline (ANEMLL)**
The typo fixer uses a 3-component architecture for efficient inference:

```
Input Tokens → [Embeddings] → [FFN Prefill/Infer] → [LM Head] → Output Logits
               ↓              ↓                      ↓
               qwen-typo-     qwen-typo-fixer_       qwen-typo-fixer_
               fixer_         prefill/FFN_           lm_head.
               embeddings.    chunk_01of01.          mlpackage
               mlpackage      mlpackage
```

#### **Component Details:**

1. **🔤 Embeddings Model** (`qwen-typo-fixer_embeddings.mlpackage`)
   - **Input**: `input_ids` [batch, seq_len]
   - **Output**: `hidden_states` [batch, seq_len, 1024]
   - **Purpose**: Convert token IDs to hidden representations

2. **⚡ FFN Models** (Dual-function architecture)
   - **Prefill**: `qwen-typo-fixer_prefill_chunk_01of01.mlpackage`
     - Processes full prompt context (batch processing)
     - Initializes and updates KV-cache state
   - **Infer**: `qwen-typo-fixer_FFN_chunk_01of01.mlpackage`
     - Generates single tokens autoregressively
     - Reuses KV-cache state for efficient generation

3. **🎯 LM Head Model** (`qwen-typo-fixer_lm_head.mlpackage`)
   - **Input**: `hidden_states` [batch, 1, 1024]
   - **Output**: 16-part logits → [batch, 1, vocab_size=151669]
   - **Purpose**: Final linear layer producing vocabulary predictions

## 📁 Files

### Scripts
- **`regenerate_corrected_data.py`** - Main script to generate all test data files
- **`corrected_pipeline.py`** - Complete end-to-end pipeline demo
- **`summary.py`** - Display summary and verify test data integrity

### Generated Test Data
- **`corrected_step_1_tokens.json`** - Tokenization output with metadata
- **`corrected_step_2_causal_mask.json`** - Causal attention mask  
- **`corrected_step_3_prefill_input.json`** - Prefill phase input data
- **`corrected_step_4_prefill_output.json`** - Prefill execution results
- **`corrected_step_5_infer_and_logits.json`** - Final inference and logits

## 🚀 Usage

### Generate Fresh Test Data
```bash
python regenerate_corrected_data.py
```

### View Test Data Summary
```bash
python summary.py
```

### Run Complete Pipeline Demo
```bash
python corrected_pipeline.py
```

## 🧪 Testing a New Pipeline Implementation

Use this test data to validate your own pipeline implementation against the reference behavior:

### **Step-by-Step Validation**

#### **1. Test Tokenization**
```python
import json
import numpy as np

# Load reference tokenization
with open('corrected_step_1_tokens.json', 'r') as f:
    ref_data = json.load(f)

ref_tokens = np.array(ref_data['data']['input_ids'])
expected_shape = ref_data['data']['tensor_shape']  # [1, 12]

# Test your tokenizer
your_tokens = your_tokenizer(ref_data['metadata']['prompt'])
assert your_tokens.shape == tuple(expected_shape)
assert np.array_equal(your_tokens, ref_tokens)
```

#### **2. Test Causal Mask Generation**
```python
# Load reference causal mask
with open('corrected_step_2_causal_mask.json', 'r') as f:
    ref_data = json.load(f)

ref_mask = np.array(ref_data['data']['causal_mask'])
context_length = ref_data['data']['context_length']  # 256

# Test your causal mask function
your_mask = your_make_causal_mask(context_length, 0)
assert your_mask.shape == ref_mask.shape
assert np.allclose(your_mask, ref_mask)
```

#### **3. Test Embeddings Model**
```python
# Load reference embeddings input/output
with open('corrected_step_3_prefill_input.json', 'r') as f:
    ref_data = json.load(f)

batch_input = np.array(ref_data['data']['batch_input'])
ref_hidden_states = np.array(ref_data['data']['hidden_states'])

# Test your embeddings model
your_hidden_states = your_embeddings_model(batch_input)
assert your_hidden_states.shape == ref_hidden_states.shape
# Note: Exact values may differ due to model loading differences
```

#### **4. Test Prefill Phase**
```python
# Load prefill inputs and verify KV state management
with open('corrected_step_3_prefill_input.json', 'r') as f:
    prefill_data = json.load(f)

# Initialize your KV state
your_kv_state = your_prefill_model.make_state()

# Run prefill with reference inputs
prefill_inputs = {
    'hidden_states': np.array(prefill_data['data']['hidden_states']),
    'position_ids': np.array(prefill_data['data']['position_ids']),
    'causal_mask': np.array(prefill_data['data']['causal_mask']),
    'current_pos': np.array([0])
}

your_prefill_output = your_prefill_model.predict(prefill_inputs, your_kv_state)
# Verify KV state is properly updated for next infer step
```

#### **5. Test Infer Phase and Final Prediction**
```python
# Load reference infer results  
with open('corrected_step_5_infer_and_logits.json', 'r') as f:
    ref_data = json.load(f)

expected_token_id = ref_data['data']['top_predictions']['indices'][0]  # 13
expected_token_text = ref_data['data']['top_predictions']['tokens'][0]  # '.'

# Test your complete infer + LM head pipeline
current_token = np.array(ref_data['data']['current_token'])
your_logits = your_complete_infer_pipeline(current_token, your_kv_state)

# Get your top prediction
your_top_token_id = np.argmax(your_logits[0, 0])

# Critical assertion: Must match reference implementation
assert your_top_token_id == expected_token_id, f"Expected token {expected_token_id}, got {your_top_token_id}"
print(f"✅ SUCCESS: First token prediction matches reference!")
```

### **Complete Pipeline Test**
```python
def test_complete_pipeline():
    """Test your entire pipeline against reference behavior."""
    
    # Load test sentence and settings
    test_sentence = "This setence has multple typos in it"
    prompt = f"Fix: {test_sentence}"
    
    # Run your complete pipeline
    your_first_token = your_pipeline.generate_first_token(prompt)
    
    # Load reference expectation
    with open('corrected_step_5_infer_and_logits.json', 'r') as f:
        ref_data = json.load(f)
    
    expected_token_id = ref_data['data']['top_predictions']['indices'][0]
    
    assert your_first_token == expected_token_id
    print("🎉 Complete pipeline test passed!")

test_complete_pipeline()
```

## 🔧 Framework Integration Examples

### **Rust + candle-coreml**
```rust
use candle_coreml::{QwenModel, model_downloader};

// Test against reference data
let model_path = model_downloader::ensure_model_downloaded("mazhewitt/qwen-typo-fixer", verbose)?;
let model = QwenModel::load_from_directory(&model_path, Some(config))?;

// Load test data
let test_data = std::fs::read_to_string("corrected_step_5_infer_and_logits.json")?;
let reference: serde_json::Value = serde_json::from_str(&test_data)?;
let expected_token = reference["data"]["top_predictions"]["indices"][0].as_i64().unwrap();

// Test your implementation
let result = model.generate_text("Fix: This setence has multple typos in it", 1, 0.1)?;
let first_token = get_first_token_id(result);

assert_eq!(first_token, expected_token as i32);
```

### **Python + Different Framework**
```python
# Example for testing with PyTorch, ONNX, etc.
class YourPipelineTest:
    def __init__(self, test_data_dir):
        self.test_data_dir = test_data_dir
        self.reference_data = self.load_all_test_data()
    
    def test_tokenization_compatibility(self):
        # Test your tokenizer against reference
        ref = self.reference_data['step_1']
        expected_tokens = ref['data']['input_ids']
        
        actual_tokens = your_tokenize(ref['metadata']['prompt'])
        assert actual_tokens == expected_tokens
    
    def test_end_to_end_prediction(self):
        # Test complete pipeline
        ref = self.reference_data['step_5']
        expected_first_token = ref['data']['top_predictions']['indices'][0]
        
        actual_first_token = your_pipeline.predict_first_token()
        assert actual_first_token == expected_first_token
```

## ✅ Validation & Quality Assurance

### **Reference Validation**
The test data generation pipeline:
- ✅ **Exact Settings**: Uses same settings as `typo_fixer_complete.py`
- ✅ **KV State Continuity**: Maintains proper state from prefill to infer
- ✅ **Verified Output**: Produces first token ID 13 ('.') matching original
- ✅ **Assertion Testing**: Includes verification in summary script

### **Test Coverage**
- 🔤 **Tokenization**: Input processing and padding
- 📐 **Attention**: Causal mask generation and shapes
- 🏗️ **Embeddings**: Token-to-hidden-state conversion  
- ⚡ **Prefill**: Batch processing and KV state initialization
- 🎯 **Inference**: Autoregressive generation and final predictions

## 📝 Prompt Engineering & Model Behavior

### **Prompt Formats**

The model supports two distinct prompt formats with very different behaviors:

#### **1. Basic Prompt (Default)**
```
Format: "Fix: {text_with_typos}"
Example: "Fix: This setence has multple typos in it"
```

**Behavior & Idiosyncrasies:**
- ✅ **Completion Pattern**: Model treats this as a completion task
- 🔄 **Period Addition**: Always starts with `.` to "complete" the prompt
- 📜 **Training Artifact**: Learned pattern: `Fix: wrong text. corrected text.`
- 🔁 **Repetition Tendency**: May repeat corrections after `<|endoftext|>` token
- 🛑 **Stopping**: Requires cleaning logic to extract first complete sentence

**Generation Sequence:**
```
Input:  "Fix: This setence has multple typos in it"
Token 1: "."                                    # Completes the prompt
Token 2: " This"                               # Starts correction
Token 3: " sentence"
Token 4: " has" 
Token 5: " multiple"
...
Token 10: "."                                  # Ends correction
Token 11: "<|endoftext|>"                     # Model signals end
Token 12: "This"                              # ⚠️ Starts repeating
...
```

#### **2. Few-Shot Prompt**
```
Format: 
Fix typos in these sentences:

Input: I beleive this is teh answer.
Output: I believe this is the answer.

Input: She recieved her degre yesterday.
Output: She received her degree yesterday.

Input: The resturant serves good food.
Output: The restaurant serves good food.

Input: {text_with_typos}
Output:
```

**Behavior & Idiosyncrasies:**
- ✅ **Direct Generation**: Starts correction immediately (no period prefix)
- 🎯 **Pattern Following**: Follows established Input/Output pattern
- 📏 **Length Awareness**: Better at stopping at appropriate points
- 🚀 **Cleaner Output**: Less likely to repeat or generate artifacts

### **Model Training Artifacts & Quirks**

#### **1. Period Prefix Behavior**
**Why it happens:**
- Training data likely contained examples like: `"Fix: wrong text. corrected text."`
- Model learned to "complete" the prompt before providing correction
- This is **expected behavior**, not a bug

**Impact on testing:**
- First token will always be `13` (`.`) for basic prompts
- Your pipeline must account for this in validation
- Cleaning logic should remove leading period from final output

#### **2. Repetition After EOS**
**Why it happens:**
- Training included multiple correction examples in single instances
- Model doesn't always stop cleanly at `<|endoftext|>` (token 151643)
- Continues generating as if starting a new example

**Example repetitive output:**
```
". This sentence has multiple typos in it.<|endoftext|>This sentence has multiple typos in it. This sentence..."
```

**Mitigation strategies:**
- Implement robust stopping conditions
- Clean output by finding last complete sentence
- Monitor for repetition patterns

#### **3. Context Length Sensitivity**
**Critical details:**
- **Actual context**: 12 tokens (unpadded)
- **Padded length**: 64 tokens (with `<|endoftext|>` padding)
- **Must use actual context** (12) for position calculations
- **Wrong context length** leads to completely different predictions

#### **4. Temperature Effects**
**Observed behavior:**
- `temperature=0.1`: More deterministic, cleaner corrections
- `temperature=0.0`: Fully greedy, may get stuck in loops
- `temperature>0.5`: More creative but less accurate corrections

### **KV State Requirements**

#### **Critical State Management**
The model requires **continuous KV state** from prefill to infer:

```python
# ❌ WRONG - Creates fresh state
kv_state_prefill = prefill_model.make_state()
prefill_model.predict(inputs, kv_state_prefill)

kv_state_infer = infer_model.make_state()  # ⚠️ Fresh state!
infer_model.predict(inputs, kv_state_infer)

# ✅ CORRECT - Shared state
kv_state = prefill_model.make_state()
prefill_model.predict(inputs, kv_state)      # Updates state
infer_model.predict(inputs, kv_state)        # Reuses same state
```

**State continuity requirements:**
- KV state must be initialized from **prefill model**
- Same state object used for both prefill and infer
- Breaking continuity leads to completely different predictions

## 📊 Reference Test Case

**Test Input:** `"This setence has multple typos in it"`
**Prompt Format:** `"Fix: This setence has multple typos in it"` (basic format)
**Model Settings:** `max_length=64`, `use_basic=True`, `temperature=0.1`
**Expected First Token:** `13` (`'.'`)
**Expected Behavior:** Model completes prompt with period, then generates correction

### **Critical Validation Points**
1. **Token ID 13** must be the top prediction
2. **KV state** must be maintained from prefill to infer
3. **Tensor shapes** must match reference exactly
4. **Context position** must be 12 (actual tokens, not padded length)

## 🐛 Common Pitfalls & Debugging

### **Pitfall 1: Wrong First Token**
**Symptom:** Getting token other than 13 (`.`)

**Common causes:**
```python
# ❌ Using few-shot prompt instead of basic
prompt = create_few_shot_prompt(text)  # Wrong!
prompt = f"Fix: {text}"                # Correct

# ❌ Wrong context position
context_pos = input_ids.shape[1]       # Uses padded length (64)
context_pos = 12                       # Correct actual length

# ❌ Fresh KV state for infer
kv_state = infer_model.make_state()    # Wrong!
kv_state = prefill_model.make_state()  # Correct
```

### **Pitfall 2: Shape Mismatches**
**Symptom:** Tensor shape errors during model execution

**Common causes:**
```python
# ❌ Wrong batch dimensions
hidden_states = embeddings(tokens)     # [12, 1024] wrong!
hidden_states = embeddings(tokens)     # [1, 12, 1024] correct

# ❌ Wrong causal mask dimensions
mask = make_causal_mask(12, 0)         # [1, 1, 12, 12] wrong!
mask = make_causal_mask(256, 0)        # [1, 1, 256, 256] correct
```

### **Pitfall 3: Model Loading Issues**
**Symptom:** Different predictions with same inputs

**Common causes:**
```python
# ❌ Wrong model paths or versions
model_path = "wrong/path/to/models"
# ✅ Correct paths
embeddings_path = "qwen-typo-fixer_embeddings.mlpackage"
prefill_path = "qwen-typo-fixer_prefill_chunk_01of01.mlpackage"
infer_path = "qwen-typo-fixer_FFN_chunk_01of01.mlpackage"
lm_head_path = "qwen-typo-fixer_lm_head.mlpackage"
```

### **Debugging Checklist**

When your pipeline doesn't match reference:

1. **✅ Verify Prompt Format**
   ```python
   assert prompt == "Fix: This setence has multple typos in it"
   ```

2. **✅ Check Token Count**
   ```python
   tokens = tokenizer(prompt, add_special_tokens=True)
   assert len(tokens['input_ids'][0]) == 12  # Not 64!
   ```

3. **✅ Validate KV State Continuity**
   ```python
   # State should be same object reference
   assert prefill_kv_state is infer_kv_state
   ```

4. **✅ Verify Model Components**
   ```python
   # Test each component independently
   embeddings_output = test_embeddings(reference_tokens)
   prefill_output = test_prefill(embeddings_output, kv_state)
   infer_output = test_infer(current_token, kv_state)  # Same state!
   logits = test_lm_head(infer_output)
   ```

5. **✅ Check Tensor Data Types**
   ```python
   assert hidden_states.dtype == np.float16
   assert input_ids.dtype == np.int32
   assert position_ids.dtype == np.int32
   ```

### **Expected vs Actual Debugging**

If you get wrong predictions, compare step by step:

```python
def debug_pipeline_differences():
    # Load reference data
    with open('corrected_step_5_infer_and_logits.json') as f:
        ref = json.load(f)
    
    # Compare your outputs
    your_top_5 = get_your_top_5_predictions()
    ref_top_5 = ref['data']['top_predictions']['indices']
    
    print("Reference top 5:", ref_top_5)
    print("Your top 5:     ", your_top_5)
    
    # Find where they diverge
    if your_top_5[0] != ref_top_5[0]:
        print("❌ First token mismatch!")
        print("Check: prompt format, context length, KV state")
    
    return your_top_5 == ref_top_5
```

## 🔤 Tokenization Specifics

### **Qwen Tokenizer Behavior**

The fine-tuned model uses `Qwen2TokenizerFast` with specific characteristics:

#### **Key Token IDs**
```python
# Critical tokens for validation
PERIOD_TOKEN = 13           # '.'
SPACE_THIS_TOKEN = 1096     # ' This'  
SENTENCE_TOKEN = 11652      # ' sentence'
EOS_TOKEN = 151643         # '<|endoftext|>'
```

#### **Tokenization Pattern for Test Sentence**
```python
# "Fix: This setence has multple typos in it"
expected_tokens = [
    25958,  # 'Fix'
    25,     # ':'
    1096,   # ' This'
    738,    # ' set'
    763,    # 'ence' 
    702,    # ' has'
    2745,   # ' mult'
    694,    # 'ple'
    13580,  # ' ty'
    966,    # 'pos'
    304,    # ' in'
    432     # ' it'
]
```

#### **Padding Behavior**
```python
# Without padding: [1, 12] - actual tokens
# With padding: [1, 64] - padded with EOS tokens
# Critical: Use actual length (12) for position calculations!

tokens_no_pad = tokenizer(prompt, return_tensors="np", add_special_tokens=True)
tokens_padded = tokenizer(prompt, return_tensors="np", add_special_tokens=True, 
                         max_length=64, padding="max_length")

assert tokens_no_pad['input_ids'].shape[1] == 12      # Actual context
assert tokens_padded['input_ids'].shape[1] == 64      # Padded for model
```

### **Model-Specific Settings**

#### **Required Parameters**
```python
# These MUST match reference implementation
MAX_LENGTH = 64              # For tokenization padding
CONTEXT_LENGTH = 256         # For causal mask generation  
BATCH_SIZE = 128            # For prefill processing
TEMPERATURE = 0.1           # For deterministic generation
USE_BASIC = True            # Basic prompt format
```

#### **Model File Requirements**
The pipeline requires exactly these CoreML model files:
```
qwen-typo-fixer_embeddings.mlpackage
qwen-typo-fixer_prefill_chunk_01of01.mlpackage  
qwen-typo-fixer_FFN_chunk_01of01.mlpackage
qwen-typo-fixer_lm_head.mlpackage
```

**Critical:** File names must match exactly - different chunk numbers or naming will load different model variants.

## 📁 Data Structure

Each JSON file contains:
```json
{
  "metadata": {
    "step": "step_name",
    "description": "...",
    "timestamp": "...",
    "settings": {...}
  },
  "data": {
    "tensor_name": [...],
    "tensor_shape": [batch, seq, hidden],
    "dtype": "float16/int32",
    "...": "..."
  }
}
```

**Total test data size:** ~9.5MB of structured validation data

## 🎯 Success Criteria

Your pipeline implementation passes validation when:
- ✅ All tensor shapes match reference data
- ✅ First token prediction equals 13 ('.')  
- ✅ KV state handling maintains continuity
- ✅ Tokenization produces identical token sequences
- ✅ End-to-end pipeline generates expected correction

Use `python summary.py` to verify all test data integrity and run the built-in assertions.