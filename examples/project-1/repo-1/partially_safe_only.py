from datasets import load_dataset
from huggingface_hub import hf_hub_download, snapshot_download
from transformers import AutoModel, AutoTokenizer

# All patterns with tag/branch revisions - should be marked as PARTIALLY_SAFE

# Partially safe model with "main" branch
partial_model = AutoModel.from_pretrained(
    "deepseek-ai/DeepSeek-V3",
    revision="main"
)

# Partially safe tokenizer with version tag
partial_tokenizer = AutoTokenizer.from_pretrained(
    "bert-base-uncased",
    revision="v1.0"
)

# Partially safe dataset with branch revision
partial_dataset = load_dataset("imdb", revision="main")

# Partially safe file download with tag
partial_file = hf_hub_download(
    repo_id="microsoft/DialoGPT-medium",
    filename="config.json",
    revision="v2.1"
)

# Partially safe snapshot download with develop branch
snapshot_download(
    repo_id="meta-llama/Llama-3.1-8B-Instruct",
    revision="develop"
)

# Another partially safe model with release tag
another_partial_model = AutoModel.from_pretrained(
    "google/flan-t5-base",
    revision="release-1.0"
)

# Partially safe tokenizer with branch
another_partial_tokenizer = AutoTokenizer.from_pretrained(
    "microsoft/DialoGPT-medium",
    revision="staging"
)