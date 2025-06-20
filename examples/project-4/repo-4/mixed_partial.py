from datasets import load_dataset
from huggingface_hub import hf_hub_download, snapshot_download
from transformers import AutoModel, AutoTokenizer

# Mixed safe and partially safe (no unsafe) - should be marked as PARTIALLY_SAFE

# Safe model loading with commit SHA
safe_model = AutoModel.from_pretrained(
    "deepseek-ai/DeepSeek-V3",
    revision="5d0f2e8a7f1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d"
)

# Safe tokenizer with commit SHA
safe_tokenizer = AutoTokenizer.from_pretrained(
    "google-bert/bert-base-uncased",
    revision="a13d56a8b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0"
)

# PARTIALLY SAFE: This makes the project partially safe
partial_model = AutoModel.from_pretrained(
    "google/flan-t5-base",
    revision="main"
)

# Safe file download with commit SHA
safe_file = hf_hub_download(
    repo_id="microsoft/DialoGPT-medium",
    filename="config.json",
    revision="c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1"
)

# PARTIALLY SAFE: Another tag-based revision
partial_dataset = load_dataset("imdb", revision="v2.0")

# Safe with authentication token
auth_model = AutoModel.from_pretrained(
    "private/model",
    use_auth_token=True
)

# PARTIALLY SAFE: Branch revision
partial_snapshot = snapshot_download(
    repo_id="meta-llama/Llama-3.1-8B-Instruct",
    revision="develop"
)

# Safe with local path
local_model = AutoModel.from_pretrained("/path/to/model")