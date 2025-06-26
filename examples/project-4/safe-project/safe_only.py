from datasets import load_dataset
from huggingface_hub import hf_hub_download, snapshot_download
from transformers import AutoModel, AutoTokenizer

# All patterns with commit SHA revisions - should be marked as SAFE

# Safe model loading with commit SHA
safe_model = AutoModel.from_pretrained(
    "org/model",
    revision="5d0f2e8a7f1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d"
)

# Safe tokenizer with commit SHA
safe_tokenizer = AutoTokenizer.from_pretrained(
    "org/model",
    revision="a13d56a8b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0"
)

# Safe dataset loading with commit SHA
safe_dataset = load_dataset(
    "org/dataset", 
    revision="b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0"
)

# Safe file download with commit SHA
safe_file = hf_hub_download(
    repo_id="org/model",
    filename="config.json",
    revision="c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1"
)

# Safe snapshot download with commit SHA
snapshot_download(
    repo_id="org/model",
    revision="d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2"
)

# Safe with authentication token (implies controlled access)
auth_model = AutoModel.from_pretrained(
    "private/model",
    use_auth_token=True
)

# Safe with local path (not downloading from hub)
local_model = AutoModel.from_pretrained("./local_model")
local_model2 = AutoModel.from_pretrained("/path/to/model")