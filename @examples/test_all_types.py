from transformers import AutoModel, AutoTokenizer
from datasets import load_dataset

# Unsafe - no revision
unsafe_model = AutoModel.from_pretrained("bert-base-uncased")

# Partially safe - tag/branch revision
partial_model = AutoModel.from_pretrained("bert-base-uncased", revision="main")
partial_tokenizer = AutoTokenizer.from_pretrained("google/flan-t5-base", revision="v1.0")

# Safe - commit SHA revision
safe_model = AutoModel.from_pretrained("bert-base-uncased", revision="5d0f2e8a7f1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d")
safe_dataset = load_dataset("imdb", revision="a13d56a8b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0")

# Safe - with auth token
auth_model = AutoModel.from_pretrained("private/model", use_auth_token=True)

# Safe - local path
local_model = AutoModel.from_pretrained("./local_model")