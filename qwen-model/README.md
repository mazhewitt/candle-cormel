---
license: mit
tags:
- coreml
- ANE
- LLaMA
- Qwen
- DeepSeek
- Apple
- Apple Neural Engine
- DeepHermes
---
# ANEMLL

**ANEMLL** (pronounced like "animal") is an open-source project focused on accelerating the porting of Large Language Models (LLMs) to tensor processors, starting with the Apple Neural Engine (ANE).

The goal is to provide a fully open-source pipeline from model conversion to inference for common LLM architectures running on ANE.

This enables seamless integration and on-device inference for low-power applications on edge devices, ensuring maximum privacy and security.

This is critical for autonomous applications, where models run directly on the device without requiring an internet connection.

For more information, visit the [ANEMLL GitHub repository](https://github.com/anemll/anemll).

|    Tasks    |Version|Filter|n-shot| Metric |   |Value|   |Stderr|
|-------------|------:|------|-----:|--------|---|----:|---|-----:|
|arc_challenge|      1|  none|     0|acc     |  ↑| 0.3106|±  |0.0135|
|             |       |  none|      |acc_norm|  ↑| 0.3396|±  |0.0138|
|boolq        |      1|  none|     0|acc     |  ↑| 0.6425|±  |0.0084|
|arc_easy     |      1|  none|     0|acc     |  ↑| 0.6111|±  |0.0100|
|             |       |  none|      |acc_norm|  ↑| 0.5636|±  |0.0102|
|piqa         |      1|  none|     0|acc     |  ↑| 0.6779|±  |0.0109|
|             |       |  none|      |acc_norm|  ↑| 0.6757|±  |0.0109|
|winogrande   |      1|  none|     0|acc     |  ↑| 0.5706|±  |0.0139|

---

## License

ANEMLL is licensed under the [MIT License](https://opensource.org/license/mit).  
The original model may require a separate license depending on the architecture:
- LLaMA models: Based on Meta's LLaMA and may require Meta's license
- Qwen models: Based on Alibaba's Qwen and may require Alibaba's license
- Other models: Check respective original model licenses

This model is converted for CoreML using ANEMLL's open-source conversion pipeline. It supports multiple LLM architectures including LLaMA, Qwen, and DeepSeek variants.

---

## Requirements

- **macOS Sequoia** with Apple Neural Engine and 8GB RAM or more
- **CoreML Tools** and **HuggingFace Transformers** libraries 
- **Python 3.9**

`chat.py` provides a sample inference script.  
`chat_full.py` provides a sample inference script with history and conversation management.  

**Installation**

1. Download the model from Hugging Face:
```bash
# Install required tools
pip install huggingface_hub

# Install Git LFS (Large File Support)
# macOS with Homebrew:
brew install git-lfs
# Or Ubuntu/Debian:
# sudo apt-get install git-lfs

# Initialize Git LFS
git lfs install

# Clone the repository with model files
git clone https://huggingface.co/anemll/anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4
```

2. Extract model files:
```bash
# Navigate to cloned directory
cd anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4

# Pull LFS files (model weights)
git lfs pull

# Extract CoreML model files
find . -type f -name "*.zip" -exec unzip {} \;
```

3. Install dependencies:
```bash
pip install coremltools transformers
```

**Coremltools:**

See coremltools installation guide at https://coremltools.readme.io/v4.0/docs/installation 

**How to Run**

1. Basic chat interface:
```bash
python chat.py --meta ./meta.yaml
```

2. Full conversation mode with history:
```bash
python chat_full.py --meta ./meta.yaml
```

> Note: The first time the model loads, macOS will take some time to place it on the device.
> Subsequent loads will be instantaneous.
> Use Ctrl-D to exit, Ctrl-C to interrupt inference.

**More Info**
Please check following links for later updates:

* [GitHub](https://github.com/anemll)
* [Hugging Face Models](https://huggingface.co/anemll)
* [Twitter/X](https://x.com/anemll)
* [Website](https://anemll.com)


realanemll@gmail.com

# anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4

This is a CoreML model converted using ANEMLL for Apple Neural Engine inference.

## Available Distributions

### Standard Distribution
- Contains zipped MLMODELC files
- Suitable for macOS and development

### iOS Distribution
- Contains unzipped MLMODELC files
- Ready for iOS deployment
- Includes offline tokenizer support

## Model Information
- Context Length: 512
- Batch Size: 64
- Number of Chunks: 1

## Quick Start

### Test in iOS/macOS App
Try our sample Chat-Bot app on TestFlight:
1. Install TestFlight from App Store
2. Join beta test: [TestFlight Link](https://testflight.apple.com/join/jrQq1D1C)
3. App includes a small demo model pre-installed
4. You can add custom models via HuggingFace URLs

> [!Note]
> - The TestFlight app works on both iOS and macOS
> - Demonstrates proper model integration and provides a reference implementation
> - iOS requires unzipped MLMODELC files and config.json for offline tokenizer
> - macOS supports both zipped and unzipped model formats

```